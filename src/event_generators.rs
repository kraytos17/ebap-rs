use crate::event::{FileReadEvent, NetworkEvent};
use crate::event_handler::EventHandler;
use std::{path::Path, sync::Arc};
use tokio::fs;

pub async fn read_file<P: AsRef<Path>>(event_handler: Arc<EventHandler>, file_path: P) {
    let path = file_path.as_ref().to_path_buf();
    let res = fs::read_to_string(&path).await;
    let event = Arc::new(FileReadEvent {
        path,
        content: res.as_ref().ok().cloned(),
        error: res.err().map(|e| e.to_string()),
    });

    event_handler.notify(event).await;
}

pub async fn fetch_url(event_handler: Arc<EventHandler>, url: String) {
    let client = reqwest::Client::new();
    let result = client.get(&url).send().await;

    let (status, response, error) = match result {
        Ok(res) => {
            let status = res.status().as_u16();
            match res.text().await {
                Ok(text) => (status, Some(text), None),
                Err(e) => (status, None, Some(e.to_string())),
            }
        }
        Err(e) => (0, None, Some(e.to_string())),
    };

    let event = Arc::new(NetworkEvent {
        url,
        status,
        response,
        error,
    });

    event_handler.notify(event).await;
}
