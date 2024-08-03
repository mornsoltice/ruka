use std::thread;
use std::time::Duration;

pub fn backoff(attempt: u32) {
    let delay = Duration::from_millis(1 << attempt);
    thread::sleep(delay);
}
