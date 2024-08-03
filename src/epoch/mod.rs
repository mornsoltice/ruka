pub mod guard;

use std::sync::Arc;

pub struct Epoch {
    current: ruka::RukaUsize,
    epoch: ruka::RukaUsize,
}

impl Epoch {
    pub fn new() -> Arc<Self> {
        Arc::new(Epoch {
            current: ruka::RukaUsize::new(0),
            epoch: ruka::RukaUsize::new(0),
        })
    }

    pub fn current_epoch(&self) -> usize {
        self.current.load()
    }

    pub fn advance(&self) {
        let new_epoch = self.epoch.fetch_add(1);
        self.current.store(new_epoch);
    }
}
