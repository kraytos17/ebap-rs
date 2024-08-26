use crate::event::{FileReadEvent, NetworkEvent};
use crate::event_handler::EventHandler;
use crate::event_listener::EventListener;
use std::borrow::Cow::Borrowed;
use std::sync::Arc;

pub struct EventManager {
    event_handler: Arc<EventHandler>,
    listeners: Vec<Arc<dyn EventListener>>,
}

impl EventManager {
    pub fn new(event_handler: Arc<EventHandler>) -> Self {
        Self {
            event_handler,
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Arc<dyn EventListener>) {
        self.listeners.push(listener);
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file_read_recv = self
            .event_handler
            .register_listener::<FileReadEvent>(Borrowed("file_read"))
            .await;

        let mut network_recv = self
            .event_handler
            .register_listener::<NetworkEvent>(Borrowed("network"))
            .await;

        loop {
            tokio::select! {
                Ok(event) = file_read_recv.recv() => {
                    for listener in &self.listeners {
                        listener.handle_event(event.clone()).await;
                    }
                }
                Ok(event) = network_recv.recv() => {
                    for listener in &self.listeners {
                        listener.handle_event(event.clone()).await;
                    }
                }
                else => break,
            }
        }
        Ok(())
    }
}
