use std::sync::{Arc,Mutex,mpsc};

#[derive(Clone)]
pub struct BiDirectionalChannel {
    sender: mpsc::Sender<String>,
    receiver: Arc<Mutex<mpsc::Receiver<String>>>
}

impl BiDirectionalChannel {
    pub fn new() -> (Self, Self) {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let rx1 = Arc::new(Mutex::new(rx1));
        let rx2 = Arc::new(Mutex::new(rx2));

        let channel1 = BiDirectionalChannel {sender:tx1,receiver:rx2};
        let channel2 = BiDirectionalChannel {sender:tx2,receiver:rx1};

        (channel1, channel2)
    }

    pub fn send(&self, message: String) -> Result<(), mpsc::SendError<String>> {
        self.sender.send(message)
    }

    pub async fn receive(&self) -> Result<String, mpsc::RecvError> {
        let lock = self.receiver.lock().unwrap();
        lock.recv()
    }
}
