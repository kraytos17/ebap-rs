use crate::event::{Event, FileReadEvent, NetworkEvent};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait EventListener: Send + Sync {
    async fn handle_event(&self, event1: Arc<dyn Event>);
}

pub struct FileReadListener;

#[async_trait]
impl EventListener for FileReadListener {
    async fn handle_event(&self, event1: Arc<dyn Event>) {
        if let Some(file_event) = event1.as_any().downcast_ref::<FileReadEvent>() {
            println!("File read: {:?}", file_event);
        }
    }
}

pub struct NetworkListener;

#[async_trait]
impl EventListener for NetworkListener {
    async fn handle_event(&self, event1: Arc<dyn Event>) {
        if let Some(nw_event) = event1.as_any().downcast_ref::<NetworkEvent>() {
            println!("Network event: {:?}", nw_event);
        }
    }
}
