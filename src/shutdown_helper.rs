use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio::task::JoinHandle;

/// Singleton for managing the shutdown signal and internal handles.
#[derive(Clone)]
pub struct ShutdownHelper {
    shutdown_tx: broadcast::Sender<()>,
    handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl ShutdownHelper {
    /// Gets the singleton instance of the `ShutdownHelper`.
    pub fn instance() -> Arc<Self> {
        static mut INSTANCE: Option<Arc<ShutdownHelper>> = None;
        static ONCE: std::sync::Once = std::sync::Once::new();

        unsafe {
            ONCE.call_once(|| {
                let (shutdown_tx, _) = broadcast::channel::<()>(1);
                INSTANCE = Some(Arc::new(ShutdownHelper {
                    shutdown_tx,
                    handles: Arc::new(Mutex::new(Vec::new())),
                }));
            });

            INSTANCE.clone().unwrap()
        }
    }

    /// Sends the shutdown signal.
    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(()); // Ignore errors if no receivers are present.
    }

    /// Subscribes to the shutdown signal.
    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.shutdown_tx.subscribe()
    }

    /// Registers a handle to be tracked.
    pub fn register_handle(&self, handle: JoinHandle<()>) {
        let mut handles = self.handles.lock().unwrap();
        handles.push(handle);
    }

    /// Awaits all tracked handles to finish.
    pub async fn await_handles(&self) {
        let mut handles = self.handles.lock().unwrap();
        while let Some(handle) = handles.pop() {
            let _ = handle.await; // Await each handle, ignoring its result.
        }
    }
}
