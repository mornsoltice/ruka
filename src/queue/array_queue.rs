pub struct ArrayQueue<T> {
    buffer: Vec<Option<T>>,
    head: crate::utils::RukaUsize,
    tail: crate::utils::RukaUsize,
    capacity: usize,
}

impl<T> ArrayQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            head: crate::utils::RukaUsize::new(0),
            tail: crate::utils::RukaUsize::new(0),
            capacity,
        }
    }

    pub fn enqueue(&self, data: T) -> Result<(), &str> {
        let tail = self.tail.load();
        let next_tail = (tail + 1) % self.capacity;
        if next_tail == self.head.load() {
            Err("Queue is full")
        } else {
            self.buffer[tail] = Some(data);
            self.tail.store(next_tail);
            Ok(())
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        let head = self.head.load();
        if head == self.tail.load() {
            None
        } else {
            let data = self.buffer[head].take();
            self.head.store((head + 1) % self.capacity);
            data
        }
    }
}
