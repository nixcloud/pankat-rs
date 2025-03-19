use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::{broadcast, Mutex};

#[derive(Debug)]
pub struct PubSubRegistry {
    channels: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
}

impl PubSubRegistry {
    /// Get the singleton instance of `PubSubRegistry`
    pub fn instance() -> &'static Arc<Self> {
        static INSTANCE: OnceLock<Arc<PubSubRegistry>> = OnceLock::new();
        INSTANCE.get_or_init(|| PubSubRegistry::new())
    }

    fn new() -> Arc<Self> {
        Arc::new(Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn get_sender_receiver_by_name(
        &self,
        name: String,
    ) -> (broadcast::Sender<String>, broadcast::Receiver<String>) {
        let mut channels = self.channels.lock().await;
        let sender = channels
            .entry(name)
            .or_insert_with(|| broadcast::channel(500).0)
            .clone();
        let receiver = sender.subscribe();
        (sender, receiver)
    }
}
