use notify::{Config, Event, EventKind, RecommendedWatcher, Watcher};
use std::path::Path;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

const DEFAULT_CHANNEL_CAPACITY: usize = 100;

pub async fn async_monitor(
    _path: impl AsRef<Path>,
    mut shutdown_rx: tokio::sync::mpsc::Receiver<()>,
) -> notify::Result<(RecommendedWatcher, mpsc::Receiver<notify::Result<Event>>)> {
    let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_CAPACITY);

    // Create watcher with custom configuration
    let watcher = RecommendedWatcher::new(
        move |res: notify::Result<Event>| {
            let tx = tx.clone();
            match res {
                Ok(event) => {
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                            if let Err(SendError(err)) = tx.blocking_send(Ok(event.clone())) {
                                eprintln!("Error sending event through channel: {:?}", err);
                            }
                        }
                        _ => {} // Ignore other event types
                    }
                }
                Err(e) => {
                    eprintln!("Watch error: {:?}", e);
                    if let Err(_) = tx.blocking_send(Err(e)) {
                        eprintln!("Error sending error event through channel");
                    }
                }
            }
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub fn handle_event(event: &Event) {
    match event.kind {
        EventKind::Create(_) => println!("File created: {:?}", event.paths),
        EventKind::Modify(_) => println!("File modified: {:?}", event.paths),
        EventKind::Remove(_) => println!("File removed: {:?}", event.paths),
        _ => {} // Ignore other events
    }
}