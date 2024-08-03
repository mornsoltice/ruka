use std::sync::Arc;

pub struct Guard {
    epoch: Arc<super::Epoch>,
    generation: usize,
}

impl Guard {
    pub fn new(epoch: Arc<super::Epoch>) -> Self {
        let generation = epoch.current_epoch();
        Self { epoch, generation }
    }

    pub fn is_valid(&self) -> bool {
        self.generation == self.epoch.current_epoch()
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        // Any cleanup necessary when the guard is dropped can be done here
    }
}
