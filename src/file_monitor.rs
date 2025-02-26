use crate::registry::PubSubRegistry;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
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
        println!("File monitor started...");

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

fn handle_event(pool: &DbPool, event: &Event) {
    let cfg = crate::config::Config::get();
    let input_path: PathBuf = cfg.input.clone();

    for path in &event.paths {
        if let Some(extension) = path.extension() {
            if extension == "mdwn" {
                if let Ok(relative_path) = path.strip_prefix(std::env::current_dir().unwrap()) {
                    let relative_article_path: PathBuf = relative_path
                        .strip_prefix(input_path.clone())
                        .unwrap()
                        .to_path_buf();
                    // let event_type = match event.kind {
                    //     EventKind::Create(_) => "📝 created",
                    //     EventKind::Modify(_) => "✏️ modified",
                    //     EventKind::Remove(_) => "🗑️ removed",
                    //     _ => return,
                    // };
                    // println!(
                    //     "  📍 Path: {} was {}",
                    //     relative_article_path.display(),
                    //     event_type
                    // );

                    let pankat_event: PankatFileMonitorEvent = PankatFileMonitorEvent {
                        kind: event.kind,
                        path: relative_article_path.to_path_buf(),
                    };
                    debounce(pool, pankat_event);
                }
            }
        }
    }
}

fn debounce(pool: &DbPool, pankat_event: PankatFileMonitorEvent) {
    // Debounce logic
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::Instant;
    use tokio::time::{sleep, Duration};
    let news_sender = PubSubRegistry::instance().register_sender("news".to_string());
    lazy_static::lazy_static! {
        static ref EVENT_CACHE: Mutex<HashMap<PankatFileMonitorEvent, Instant>> = Mutex::new(HashMap::new());
    }

    const DEBOUNCE_DURATION: Duration = Duration::from_millis(50);

    {
        let mut cache = EVENT_CACHE.lock().unwrap();
        if !cache.contains_key(&pankat_event) {
            cache.insert(pankat_event.clone(), Instant::now() + DEBOUNCE_DURATION);
        }
    }

    let pool = pool.clone();

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
                        println!("Processing cached event for: {}", event.path.display());
                        let mut conn = pool
                            .get()
                            .expect("Failed to get a connection from the pool");
                        match crate::articles::file_monitor_articles_change(&mut conn, &event) {
                            Ok(html) => {
                                //println!("sending the good news: {}", html);
                                match news_sender.send(html) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        print!("Error sending news: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                print!("file_monitor_articles_change Error: {:?}", e);
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
        println!("end of loop: tokio::spawn(async move");
    });
}
