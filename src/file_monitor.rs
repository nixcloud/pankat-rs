use crate::registry::PubSubRegistry;
use crate::render::render_file;
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

    let news_sender = PubSubRegistry::instance().register_sender("news".to_string());

    // Spawn monitoring task
    let handle = tokio::spawn(async move {
        println!("File monitor started...");

        tokio::select! {
            _ = async {
                while let Some(event) = rx.recv().await {
                    match event {
                        Ok(event) => handle_event(&event, &news_sender),
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

fn handle_event(event: &Event, news_sender: &Sender<String>) {
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
                    debounce(path_string, &news_sender);
                }
            }
        }
    }
}

fn debounce(input: String, news_sender: &Sender<String>) {
    // Debounce logic
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::Instant;
    use tokio::time::{sleep, Duration};

    lazy_static::lazy_static! {
        static ref EVENT_CACHE: Mutex<HashMap<String, Instant>> = Mutex::new(HashMap::new());
    }

    const DEBOUNCE_DURATION: Duration = Duration::from_millis(5000);

    {
        let mut cache = EVENT_CACHE.lock().unwrap();
        if !cache.contains_key(&input) {
            cache.insert(input.clone(), Instant::now() + DEBOUNCE_DURATION);
        }
    }

    tokio::spawn(async move {
        loop {
            // Acquire lock to access EVENT_CACHE
            let mut cache = EVENT_CACHE.lock().unwrap();

            // Determine the shortest duration to sleep
            if let Some((&ref key, &instant)) = cache.iter().min_by_key(|&(_, &instant)| instant) {
                let now = Instant::now();

                // If the time has passed, remove the entry and proceed to handle it
                if instant <= now {
                    cache.remove(&key.clone());
                    println!("Processing cached event for: {}", key);

                    // Placeholder for processing logic
                    if let Ok(result) = render_file(key.clone()) {
                        let _ = news_sender.send(result);
                    }
                } else {
                    // Calculate sleep duration
                    let duration = instant.duration_since(now);

                    // Release the lock before sleeping
                    drop(cache);

                    // Sleep for the calculated duration
                    sleep(duration).await;
                }
            } else {
                // If EVENT_CACHE is empty, break out of the loop
                break;
            }
        }
    });

    // println!("Dispatching event for: {}", input);
    // match render_file(input.clone()) {
    //     Ok(result) => {
    //         news_sender.send(result);
    //     }
    //     Err(_) => {}
    // }
}
