use tokio::sync::broadcast::{channel, Sender, error::{SendError, RecvError}};

#[derive(Clone)]
pub struct BroadcastChannel {
    sender: Sender<String>,
}

impl BroadcastChannel {
    pub fn new() -> Self {
        let (tx, _) = channel(10);
        let channel = BroadcastChannel { sender: tx };
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
