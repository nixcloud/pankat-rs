// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::sync::mpsc::{self, Sender, Receiver};
// use std::thread;

// // The message broker is implemented as a singleton.
// struct PubSub {
//     subscribers: HashMap<String, Vec<Sender<String>>>,
// }

// impl PubSub {
//     // Get a mutable reference to the singleton instance.
//     fn instance() -> Arc<Mutex<Self>> {
//         static INSTANCE: once_cell::sync::OnceCell<Arc<Mutex<PubSub>>> = once_cell::sync::OnceCell::new();
//         INSTANCE.get_or_init(|| Arc::new(Mutex::new(PubSub { subscribers: HashMap::new() }))).clone()
//     }

//     // Subscribe to a topic.
//     fn subscribe(&mut self, topic: String) -> Receiver<String> {
//         let (tx, rx) = mpsc::channel();

//         // Add the subscriber to the topic.
//         self.subscribers
//             .entry(topic)
//             .or_insert_with(Vec::new)
//             .push(tx);

//         rx
//     }

//     // Publish a message to a topic.
//     fn publish(&mut self, topic: &str, message: String) {
//         if let Some(subscribers) = self.subscribers.get_mut(topic) {
//             for subscriber in subscribers {
//                 if subscriber.send(message.clone()).is_err() {
//                     // Remove the subscriber if sending fails.
//                     // This may occur if the receiver is no longer active.
//                     println!("Subscriber disconnected from topic '{}'", topic);
//                 }
//             }
//         }
//     }
// }

// fn main() {
//     let pubsub = PubSub::instance();

//     // Subscribe to the "news" topic.
//     let rx = {
//         let mut instance = pubsub.lock().unwrap();
//         instance.subscribe("news".to_string())
//     };

//     // Start a thread to listen for messages.
//     thread::spawn(move || {
//         for msg in rx {
//             println!("Received: {}", msg);
//         }
//     });

//     // Publish a message to the "news" topic.
//     {
//         let mut instance = pubsub.lock().unwrap();
//         instance.publish("news", "Breaking News: Rust is awesome!".to_string());
//     }

//     // Allow the listener thread to process the message.
//     thread::sleep(std::time::Duration::from_millis(100));
// }
