use crate::event::{Event, FileReadEvent, NetworkEvent};
use std::sync::Arc;

pub trait EventListener: Send + Sync {
    async fn handle_event(&self, event1: Arc<dyn Event>);
}

pub struct FileReadListener;

impl EventListener for FileReadListener {
    async fn handle_event(&self, event1: Arc<dyn Event>) {
        if let Some(file_event) = event1.as_any().downcast_ref::<FileReadEvent>() {
            println!("Event Type: File Read");
            println!("-------------------");
            println!("Path: {:?}", file_event.path);
            println!(
                "Content:\n{}",
                file_event.content.as_deref().unwrap_or("None")
            );
            println!("Error: {:?}", file_event.error);
            println!();
        }
    }
}

pub struct NetworkListener;

impl EventListener for NetworkListener {
    async fn handle_event(&self, event1: Arc<dyn Event>) {
        if let Some(nw_event) = event1.as_any().downcast_ref::<NetworkEvent>() {
            println!("Event Type: Network");
            println!("-------------------");
            println!("URL: {}", nw_event.url);
            println!("Status: {}", nw_event.status);
            println!("Response:");
            if let Some(response) = &nw_event.response {
                println!("{}", response);
            } else {
                println!("None");
            }
            println!("Error: {:?}", nw_event.error);
            println!();
        }
    }
}

pub enum EventListenerEnum {
    FileRead(FileReadListener),
    Network(NetworkListener),
}

impl EventListenerEnum {
    pub async fn handle_event(&self, event1: Arc<dyn Event>) {
        match self {
            Self::FileRead(listener) => listener.handle_event(event1).await,
            Self::Network(listener) => listener.handle_event(event1).await,
        }
    }
}
