use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct BoundedChannel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> BoundedChannel<T> {
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::sync_channel(capacity);
        Self { sender, receiver }
    }

    pub fn send(&self, data: T) -> Result<(), mpsc::SendError<T>> {
        self.sender.send(data)
    }

    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        self.receiver.recv()
    }
}
