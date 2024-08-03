use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct CachePadded<T> {
    inner: UnsafeCell<MaybeUninit<T>>,
    _pad: [u8; 64], // Assume a cache line size of 64 bytes.
}

impl<T> CachePadded<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(MaybeUninit::new(value)),
            _pad: [0; 64],
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &(*self.inner.get()).assume_init() }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut (*self.inner.get()).assume_init_mut() }
    }
}
