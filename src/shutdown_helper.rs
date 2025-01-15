use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

/// Singleton for managing the shutdown signal.
#[derive(Clone)]
pub struct ShutdownHelper {
    shutdown_tx: broadcast::Sender<()>,
}

impl ShutdownHelper {
    /// Gets the singleton instance of the `ShutdownHelper`.
    pub fn instance() -> Arc<Self> {
        static mut INSTANCE: Option<Arc<ShutdownHelper>> = None;
        static ONCE: std::sync::Once = std::sync::Once::new();

        unsafe {
            ONCE.call_once(|| {
                let (shutdown_tx, _) = broadcast::channel::<()>(1);
                INSTANCE = Some(Arc::new(ShutdownHelper { shutdown_tx }));
            });

            INSTANCE.clone().unwrap()
        }
    }

    /// Sends the shutdown signal.
    pub fn shutdown(&self) {
        // It's fine to ignore the result since receivers may have already been dropped.
        let _ = self.shutdown_tx.send(());
    }

    /// Subscribes to the shutdown signal.
    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.shutdown_tx.subscribe()
    }
}
