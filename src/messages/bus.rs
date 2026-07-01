use tokio::sync::broadcast::{self, Receiver, Sender};

use super::event::Event;

#[derive(Clone)]
pub struct EventBus {
    sender: Sender<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        let (s, _) = broadcast::channel(1024);
        Self { sender: s }
    }

    pub fn publish(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> Receiver<Event> {
        self.sender.subscribe()
    }
}
