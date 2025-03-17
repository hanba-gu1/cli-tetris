use std::time::Duration;

use crate::event::{mino_operation::MinoOperation, Event, EventSender};

pub struct FallingClock {
    event_sender: EventSender,
    handle: Option<tokio::task::JoinHandle<()>>,
}
impl FallingClock {
    pub fn new(event_sender: EventSender) -> Self {
        Self {
            event_sender,
            handle: None,
        }
    }
    pub async fn start(&mut self, duration: Duration) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
        let event_sender = self.event_sender.clone();
        let handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(duration).await;
                let _ = event_sender
                    .send(Event::MinoOperation(MinoOperation::Fall))
                    .await;
            }
        });
        self.handle = Some(handle);
    }
    pub async fn stop(&self) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
    }
}
