use async_broadcast::{broadcast, Receiver, Sender};
//use futures_lite::{future::block_on, stream::StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Clone)]
pub struct PubSubRegistry {
    channels: Arc<Mutex<HashMap<String, (Sender<String>, Receiver<String>)>>>,
}

impl PubSubRegistry {
    /// Get the singleton instance of `PubSubRegistry`
    pub fn instance() -> &'static Self {
        println!("Creating new instance of PubSubRegistry");
        static INSTANCE: OnceLock<PubSubRegistry> = OnceLock::new();
        INSTANCE.get_or_init(PubSubRegistry::new)
    }

    fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_sender_receiver_by_name(
        &self,
        name: String,
    ) -> Result<(Sender<String>, Receiver<String>), String> {
        println!("get_sender_receiver_by_name channel: {}", name);

        let mut channels = self.channels.lock().map_err(|_| "Mutex lock poisoned")?;
        match channels.get(&name) {
            Some((s, r)) => Ok((s.clone(), r.clone())),
            None => {
                let (s, r) = broadcast::<String>(5);
                channels.insert(name.to_string(), (s.clone(), r.clone()));
                Ok((s, r))
            }
        }
    }
}
