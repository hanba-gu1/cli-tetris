use std::time::Duration;

use crate::event::{Event, EventSender};

pub struct Timer {
    event_sender: EventSender,
    handle: Option<tokio::task::JoinHandle<()>>,
}
impl Timer {
    pub fn new(event_sender: EventSender) -> Self {
        Self {
            event_sender,
            handle: None,
        }
    }
    pub fn start(&mut self, time: Duration, event: Event) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
        let sender = self.event_sender.clone();
        self.handle = Some(tokio::spawn(async move {
            tokio::time::sleep(time).await;
            let _ = sender.send(event).await;
        }));
    }
}
