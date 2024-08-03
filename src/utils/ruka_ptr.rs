use std::sync::atomic::{AtomicPtr, Ordering};

pub struct RukaPtr<T> {
    value: AtomicPtr<T>,
}

impl<T> RukaPtr<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self {
            value: AtomicPtr::new(ptr),
        }
    }

    pub fn load(&self) -> *mut T {
        self.value.load(Ordering::Acquire)
    }

    pub fn store(&self, ptr: *mut T) {
        self.value.store(ptr, Ordering::Release)
    }

    pub fn compare_exchange(&self, old: *mut T, new: *mut T) -> Result<(), *mut T> {
        self.value
            .compare_exchange(old, new, Ordering::Release, Ordering::Relaxed)
    }
}
