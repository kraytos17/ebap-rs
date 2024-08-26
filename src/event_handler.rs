use crate::event::Event;
use std::{borrow::Cow, collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};

#[derive(Debug)]
pub struct EventHandler {
    listeners: Arc<RwLock<EventHandlerHM>>,
}

type EventHandlerHM = HashMap<Cow<'static, str>, broadcast::Sender<Arc<dyn Event>>>;

impl EventHandler {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_listener<E: Event + 'static>(
        &self,
        event_type: Cow<'static, str>,
    ) -> broadcast::Receiver<Arc<dyn Event>> {
        let mut listeners = self.listeners.write().await;
        match listeners.get(&event_type) {
            Some(sender) => sender.subscribe(),
            None => {
                let (tx, rx) = broadcast::channel(100);
                listeners.insert(event_type, tx);

                rx
            }
        }
    }

    pub async fn notify(&self, event: Arc<dyn Event>) {
        let listeners = self.listeners.read().await;
        if let Some(sender) = listeners.get(event.event_type()) {
            if let Err(e) = sender.send(event) {
                eprintln!("Failed to send event : {e}");
            }
        }
    }
}
