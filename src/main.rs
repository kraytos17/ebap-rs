use std::sync::Arc;

use event_generators::{fetch_url, read_file};
use event_handler::EventHandler;
use event_listener::{FileReadListener, NetworkListener};
use event_manager::EventManager;

mod event;
mod event_generators;
mod event_handler;
mod event_listener;
mod event_manager;

#[tokio::main]
async fn main() {
    let event_handler = Arc::new(EventHandler::new());
    let mut event_manager = EventManager::new(event_handler.clone());

    event_manager.add_listener(Arc::new(FileReadListener));
    event_manager.add_listener(Arc::new(NetworkListener));

    tokio::spawn(async move {
        let _ = event_manager.run().await;
    });

    read_file(event_handler.clone(), "test.txt").await;
    fetch_url(
        event_handler.clone(),
        "https://en.wikipedia.org/wiki/Black-throated_loon".to_string(),
    )
    .await;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}
