use tokio::sync::mpsc;

pub mod mino_operation;

#[derive(Clone, Copy)]
pub enum Event {
    End,
    DisplayAll,
    MinoOperation(mino_operation::MinoOperation),
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
    pub async fn send(&self, event: Event) {
        let _ = self.sender.send(event).await;
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

#[derive(Clone)]
pub struct EventSender {
    sender: mpsc::Sender<Event>,
}
impl EventSender {
    pub async fn send(&self, event: Event) {
        let _ = self.sender.send(event).await;
    }
}
