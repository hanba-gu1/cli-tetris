use tokio::sync::mpsc;

pub(crate) mod mino_operation;

#[derive(Clone, Copy)]
pub(super) enum Event {
    End,
    DisplayAll,
    MinoOperation(mino_operation::MinoOperation),
}

pub(super) struct EventManager {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
}
impl EventManager {
    pub(super) fn new() -> Self {
        let (sender, receiver) = mpsc::channel(128);
        Self { sender, receiver }
    }
    pub(super) async fn send(&self, event: Event) {
        let _ = self.sender.send(event).await;
    }
    pub(super) async fn recv(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
    pub(super) fn sender(&self) -> EventSender {
        EventSender {
            sender: self.sender.clone(),
        }
    }
}

#[derive(Clone)]
pub(super) struct EventSender {
    sender: mpsc::Sender<Event>,
}
impl EventSender {
    pub(super) async fn send(&self, event: Event) {
        let _ = self.sender.send(event).await;
    }
}
