use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Scoped<T> {
    tx: mpsc::Sender<T>,
    rx: mpsc::Receiver<T>,
}

impl<T> Scoped<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let result = f();
            tx.send(result).unwrap();
        });
        Self { tx, rx }
    }

    pub fn join(self) -> T {
        self.rx.recv().unwrap()
    }
}
