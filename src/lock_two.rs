use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::Lock;

pub static THREAD_IDS: AtomicUsize = AtomicUsize::new(0);
thread_local! {
    static THREAD_ID: Cell<usize> = Cell::new(THREAD_IDS.fetch_add(1, Ordering::Relaxed));
}

pub struct LockTwo {
    victim: *mut usize,
}

unsafe impl Send for LockTwo {}

impl Clone for LockTwo {
    fn clone(&self) -> Self {
        Self {
            victim: self.victim.clone(),
        }
    }
}

impl LockTwo {
    pub fn new() -> Self {
        let victim = Box::into_raw(Box::new(0));

        Self { victim }
    }
}

impl Lock for LockTwo {
    fn lock(&self) {
        let id = THREAD_ID.get();

        unsafe {
            (*self.victim) = id;

            while (*self.victim) == id {}
        }
    }

    fn unlock(&self) {}
}
