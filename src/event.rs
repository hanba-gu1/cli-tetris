use tokio::sync::mpsc;

#[derive(Clone)]
pub enum Event {
    End,
}

pub struct EventManager {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
}
impl EventManager {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(128);
        Self { sender, receiver }
    }
    pub async fn recv(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
    pub fn sender(&self) -> EventSender {
        EventSender {
            sender: self.sender.clone(),
        }
    }
}

pub struct EventSender {
    sender: mpsc::Sender<Event>,
}
impl EventSender {
    pub async fn send(&self, event: Event) {
        let _ = self.sender.send(event).await;
    }
}
