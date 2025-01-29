use crate::registry::PubSubRegistry;
use crate::renderer::pandoc::render_file;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::task::JoinHandle;

const DEFAULT_CHANNEL_CAPACITY: usize = 100;

pub fn spawn_async_monitor(
    path: impl AsRef<Path>,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<JoinHandle<()>, Box<dyn std::error::Error + Send + Sync>> {
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
                        Ok(event) => handle_event(&event),
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

fn handle_event(event: &Event) {
    let event_type = match event.kind {
        EventKind::Create(_) => "📝 created",
        EventKind::Modify(_) => "✏️ modified",
        EventKind::Remove(_) => "🗑️ removed",
        _ => return, // Ignore other events
    };

    for path in &event.paths {
        if let Some(extension) = path.extension() {
            if extension == "md" {
                if let Ok(relative_path) = path.strip_prefix(std::env::current_dir().unwrap()) {
                    println!("  📍 Path: {} was {}", relative_path.display(), event_type);
                    let path_string = path.to_string_lossy().into_owned();
                    debounce(path_string);
                }
            }
        }
    }
}

fn debounce(input: String) {
    // Debounce logic
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::Instant;
    use tokio::time::{sleep, Duration};
    let news_sender = PubSubRegistry::instance().register_sender("news".to_string());
    lazy_static::lazy_static! {
        static ref EVENT_CACHE: Mutex<HashMap<String, Instant>> = Mutex::new(HashMap::new());
    }

    const DEBOUNCE_DURATION: Duration = Duration::from_millis(50);

    {
        let mut cache = EVENT_CACHE.lock().unwrap();
        if !cache.contains_key(&input) {
            cache.insert(input.clone(), Instant::now() + DEBOUNCE_DURATION);
        }
    }

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
                Some((key, instant)) => {
                    let now = Instant::now();
                    if instant <= now {
                        // Remove the entry
                        {
                            let mut cache = EVENT_CACHE.lock().unwrap();
                            cache.remove(&key);
                        }
                        println!("Processing cached event for: {}", key);
                        if let Ok(result) = render_file(key) {
                            let _ = news_sender.send(result);
                        }
                    } else {
                        let duration = instant.duration_since(now);
                        sleep(duration).await;
                    }
                }
                None => break,
            }
        }
    });
}
