use std::sync::atomic::{AtomicUsize, Ordering};

pub struct RukaUsize {
    value: AtomicUsize,
}

impl RukaUsize {
    pub fn new(value: usize) -> Self {
        Self {
            value: AtomicUsize::new(value),
        }
    }

    pub fn load(&self) -> usize {
        self.value.load(Ordering::Acquire)
    }

    pub fn store(&self, value: usize) {
        self.value.store(value, Ordering::Release)
    }

    pub fn fetch_add(&self, value: usize) -> usize {
        self.value.fetch_add(value, Ordering::Release)
    }
}
