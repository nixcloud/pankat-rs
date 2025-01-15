use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock};

// Implement required traits for Sender
impl<T> Hash for Sender<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let addr = self as *const _ as usize;
        addr.hash(state);
    }
}

impl<T> PartialEq for Sender<T> {
    fn eq(&self, other: &Self) -> bool {
        let addr1 = self as *const _ as usize;
        let addr2 = other as *const _ as usize;
        addr1 == addr2
    }
}

impl<T> Eq for Sender<T> {}

#[derive(Clone)]
pub struct PubSubRegistry {
    channels: Arc<Mutex<HashMap<String, HashSet<Sender<String>>>>>,
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
        let (tx, rx) = channel();
        let mut channels = self.channels.lock().unwrap();
        channels.entry(channel).or_default().insert(tx);
        rx
    }

    /// Register a client as a sender for a specific channel
    pub fn register_sender(&self, channel: String) -> Sender<String> {
        let registry = self.clone();
        let (tx, rx) = channel();

        // Spawn a thread to listen for messages and broadcast them
        std::thread::spawn(move || {
            for message in rx {
                registry.broadcast(&channel, message);
            }
        });

        tx
    }

    /// Broadcast a message to all receivers of the channel
    fn broadcast(&self, channel: &str, message: String) {
        let channels = self.channels.lock().unwrap();
        if let Some(receivers) = channels.get(channel) {
            for receiver in receivers {
                let _ = receiver.send(message.clone());
            }
        }
    }
}