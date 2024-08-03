use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Condvar;
use std::sync::Mutex;

pub struct Parker {
    parked: AtomicBool,
    condvar: Condvar,
    mutex: Mutex<()>,
}

impl Parker {
    pub fn new() -> Self {
        Self {
            parked: AtomicBool::new(false),
            condvar: Condvar::new(),
            mutex: Mutex::new(()),
        }
    }

    pub fn park(&self) {
        let _lock = self.mutex.lock().unwrap();
        self.parked.store(true, Ordering::Release);
        self.condvar.wait(&self.mutex.lock().unwrap()).unwrap();
    }

    pub fn unpark(&self) {
        self.parked.store(false, Ordering::Release);
        self.condvar.notify_one();
    }
}
