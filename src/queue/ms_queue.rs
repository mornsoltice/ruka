use std::ptr;
use std::sync::Arc;

pub struct Node<T> {
    data: Option<T>,
    next: ruka::RukaPtr<Node<T>>,
}

pub struct MSQueue<T> {
    head: ruka::RukaPtr<Node<T>>,
    tail: ruka::RukaPtr<Node<T>>,
}

impl<T> MSQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            data: None,
            next: ruka::RukaPtr::new(ptr::null_mut()),
        }));
        Self {
            head: ruka::RukaPtr::new(dummy),
            tail: ruka::RukaPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data: Some(data),
            next: ruka::RukaPtr::new(ptr::null_mut()),
        }));
        loop {
            let tail = self.tail.load();
            let next = unsafe { (*tail).next.load() };
            if next.is_null() {
                if unsafe { (*tail).next.compare_exchange(ptr::null_mut(), new_node) }.is_ok() {
                    self.tail.store(new_node);
                    return;
                }
            } else {
                self.tail.store(next);
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load();
            let tail = self.tail.load();
            let next = unsafe { (*head).next.load() };
            if head == tail {
                if next.is_null() {
                    return None;
                }
                self.tail.store(next);
            } else {
                if let Some(data) = unsafe { (*next).data.take() } {
                    self.head.store(next);
                    return Some(data);
                }
            }
        }
    }
}
