use std::sync::{Arc,Mutex,mpsc};
use tokio::sync::broadcast::{channel, Sender, Receiver, error::{SendError, RecvError}};

#[derive(Clone)]
pub struct BiDirectionalChannel {
    sender: Sender<String>,
}

impl BiDirectionalChannel {
    pub fn new() -> Self {
        let (tx, _) = channel(10);
        let channel = BiDirectionalChannel { sender: tx };
        channel
    }

    pub fn send(&self, message: String) -> Result<usize, SendError<String>> {
        self.sender.send(message)
    }

    pub async fn receive(&self) -> Result<String, RecvError> {
        let mut rx = self.sender.subscribe();
        rx.recv().await
    }
}
