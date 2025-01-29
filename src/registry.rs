use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock};

// Custom wrapper to make Sender hashable
#[derive(Clone)]
struct HashableSender<T>(Sender<T>);

impl<T> Hash for HashableSender<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let addr = &self.0 as *const _ as usize;
        addr.hash(state);
    }
}

impl<T> PartialEq for HashableSender<T> {
    fn eq(&self, other: &Self) -> bool {
        let addr1 = &self.0 as *const _ as usize;
        let addr2 = &other.0 as *const _ as usize;
        addr1 == addr2
    }
}

impl<T> Eq for HashableSender<T> {}

#[derive(Clone)]
pub struct PubSubRegistry {
    channels: Arc<Mutex<HashMap<String, HashSet<HashableSender<String>>>>>,
}

impl PubSubRegistry {
    /// Get the singleton instance of PubSubRegistry
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<PubSubRegistry> = OnceLock::new();
        INSTANCE.get_or_init(|| PubSubRegistry::new())
    }

    /// Create a new PubSubRegistry (private)
    fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a client as a receiver for a specific channel
    pub fn register_receiver(&self, channel: String) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();
        let mut channels = self.channels.lock().unwrap();
        channels
            .entry(channel)
            .or_default()
            .insert(HashableSender(tx));
        rx
    }

    /// Register a client as a sender for a specific channel
    pub fn register_sender(&self, channel: String) -> Sender<String> {
        let registry = self.clone();
        let (tx, rx) = mpsc::channel();

        // Spawn a thread to listen for messages and broadcast them
        std::thread::spawn(move || {
            while let Ok(message) = rx.recv() {
                registry.broadcast(&channel, message);
            }
        });

        tx
    }

    /// Broadcast a message to all receivers of the channel
    fn broadcast(&self, channel: &str, message: String) {
        let mut channels = self.channels.lock().unwrap();
        if let Some(receivers) = channels.get_mut(channel) {
            // Remove disconnected receivers and send to connected ones
            receivers.retain(|receiver| receiver.0.send(message.clone()).is_ok());
        }
    }
}
