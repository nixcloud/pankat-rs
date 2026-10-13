use crate::registry::PubSubRegistry;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use notify::event::{ModifyKind, RenameMode};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::task::JoinHandle;
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
#[derive(Debug, Clone)]
pub struct PankatFileMonitorEvent {
    pub kind: EventKind,
    pub path: PathBuf,
    /// set on the old path of a rename, clients of the old article get a link to the new one
    pub renamed_to: Option<PathBuf>,
}

impl PartialEq for PankatFileMonitorEvent {
    fn eq(&self, other: &Self) -> bool {
        //println!("Comparing {:?} to {:?}", self, other);
        self.path == other.path
    }
}

impl std::hash::Hash for PankatFileMonitorEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        //println!("Hashing {:?}", self);
        self.path.hash(state);
    }
}

impl Eq for PankatFileMonitorEvent {}

const DEFAULT_CHANNEL_CAPACITY: usize = 100;

pub fn spawn_async_monitor(
    pool: DbPool,
    path: impl AsRef<Path>,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<JoinHandle<()>, Box<dyn std::error::Error + Send + Sync>> {
    println!("Monitoring input directory: {}", path.as_ref().display());

    // Store the path for cleanup
    let watch_path = path.as_ref().to_owned();

    // Create a Watcher instance
    let (tx, mut rx) = mpsc::channel::<Result<Event, notify::Error>>(DEFAULT_CHANNEL_CAPACITY);
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.blocking_send(res);
        },
        Config::default(),
    )?;

    // Start watching the path
    watcher.watch(&watch_path, RecursiveMode::Recursive)?;

    // Spawn a cleanup task to handle shutdown signal
    let watcher_cleanup = Arc::new(Mutex::new(watcher));
    let cleanup_watcher = watcher_cleanup.clone();

    // Spawn monitoring task
    let handle = tokio::spawn(async move {
        println!("File monitor started on path: {}", watch_path.display());

        tokio::select! {
            _ = async {
                while let Some(event) = rx.recv().await {
                    match event {
                        Ok(event) => handle_event(&pool, &event),
                        Err(e) => eprintln!("Watch error: {:?}", e),
                    }
                }
            } => {
                println!("File monitor channel closed, shutting down...");
            }
            _ = shutdown_rx.recv() => {
                println!("Shutdown signal received in file monitor. Cleaning up...");
                let mut watcher = cleanup_watcher.lock().await;
                if let Err(e) = watcher.unwatch(&watch_path) {
                    eprintln!("Error unwatching path during shutdown: {:?}", e);
                }
                println!("File monitor cleanup completed.");
            }
        }
    });

    Ok(handle)
}

/// path of an article relative to the input directory, None for anything that isn't one
fn article_path(path: &Path) -> Option<PathBuf> {
    if path.extension()? != "mdwn" {
        return None;
    }
    let cfg = crate::config::Config::get();
    let relative_path = path.strip_prefix(std::env::current_dir().ok()?).ok()?;
    relative_path
        .strip_prefix(&cfg.input)
        .ok()
        .map(Path::to_path_buf)
}

fn handle_event(pool: &DbPool, event: &Event) {
    // after Name(From) and Name(To) for the single paths, a rename ends with Name(Both)
    // carrying [old, new], which tags the pending removal of the old path
    if event.kind == EventKind::Modify(ModifyKind::Name(RenameMode::Both)) {
        if let [from, to] = event.paths.as_slice() {
            if let (Some(from), Some(to)) = (article_path(from), article_path(to)) {
                debounce(
                    pool,
                    PankatFileMonitorEvent {
                        kind: event.kind,
                        path: from,
                        renamed_to: Some(to),
                    },
                );
            }
        }
        return;
    }

    for path in &event.paths {
        if let Some(relative_article_path) = article_path(path) {
            debounce(
                pool,
                PankatFileMonitorEvent {
                    kind: event.kind,
                    path: relative_article_path,
                    renamed_to: None,
                },
            );
        }
    }
}

fn debounce(pool: &DbPool, pankat_event: PankatFileMonitorEvent) {
    // Debounce logic
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::Instant;
    use tokio::time::{sleep, Duration};

    lazy_static::lazy_static! {
        static ref EVENT_CACHE: Mutex<HashMap<PankatFileMonitorEvent, Instant>> = Mutex::new(HashMap::new());
    }

    const DEBOUNCE_DURATION: Duration = Duration::from_millis(50);

    {
        let mut cache = EVENT_CACHE.lock().unwrap();
        // events are equal by path, a rename target must survive in the pending one
        let pending = cache.remove_entry(&pankat_event);
        match pending {
            Some((pending_event, instant)) => {
                let renamed_to = pankat_event.renamed_to.or(pending_event.renamed_to.clone());
                cache.insert(
                    PankatFileMonitorEvent {
                        renamed_to,
                        ..pending_event
                    },
                    instant,
                );
            }
            None => {
                cache.insert(pankat_event, Instant::now() + DEBOUNCE_DURATION);
            }
        }
    }

    let pool = pool.clone();
    // println!(
    //     "Free connections: {}. Connections in use: {}.",
    //     pool.state().idle_connections,
    //     pool.state().connections - pool.state().idle_connections
    // );
    let lock = Arc::new(tokio::sync::Mutex::new(()));
    tokio::spawn(async move {
        loop {
            let next_event = {
                let cache = EVENT_CACHE.lock().unwrap();
                cache
                    .iter()
                    .min_by_key(|&(_, &instant)| instant)
                    .map(|(k, &v)| (k.clone(), v))
            };

            match next_event {
                Some((event, instant)) => {
                    let now = Instant::now();
                    if instant <= now {
                        // Remove the entry
                        {
                            let mut cache = EVENT_CACHE.lock().unwrap();
                            cache.remove(&event);
                        }
                        let _guard = lock.lock().await;

                        println!("Processing cached event for: {}", event.path.display());
                        match crate::articles::file_monitor_articles_change(
                            &mut pool.get().unwrap(),
                            &event,
                        ) {
                            Ok(updates) => {
                                // each update goes to the clients registered for that article,
                                // which is not necessarily the article that triggered this task
                                for (src_file_name, data) in updates {
                                    let (sender, _) = PubSubRegistry::instance()
                                        .get_sender_receiver_by_name(src_file_name)
                                        .await;
                                    // send only fails when nobody is subscribed
                                    let _ = sender.send(data);
                                }
                            }
                            Err(e) => {
                                println!("file_monitor_articles_change Error: {:?}", e);
                            }
                        }
                    } else {
                        let duration = instant.duration_since(now);
                        sleep(duration).await;
                    }
                }
                None => break,
            }
        }
        //println!("end of loop: tokio::spawn(async move");
    });
}
