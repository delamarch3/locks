use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    lock: Mutex<()>,
    value: RefCell<usize>,
}

unsafe impl Sync for Counter {}
unsafe impl Send for Counter {}

impl Counter {
    // It would be better to put the value in the mutex or use
    // one of the atomic types from std::sync.
    pub fn new() -> Self {
        Self {
            lock: Mutex::new(()),
            value: RefCell::new(0),
        }
    }

    pub fn get_and_increment(&self) -> usize {
        let _ = self.lock.lock();

        let tmp = *self.value.borrow();
        *self.value.borrow_mut() += 1;
        tmp
    }
}

fn main() {
    let counter = Arc::new(Counter::new());

    let counter_a = counter.clone();
    let jh1 = thread::spawn(move || {
        for _ in 0..100 {
            let val = counter_a.get_and_increment();

            eprintln!("{val}");
        }
    });

    let counter_b = counter.clone();
    let jh2 = thread::spawn(move || {
        for _ in 0..100 {
            let val = counter_b.get_and_increment();

            eprintln!("{val}");
        }
    });

    let _ = jh1.join();
    let _ = jh2.join();
}
