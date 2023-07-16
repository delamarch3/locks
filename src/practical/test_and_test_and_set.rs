use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::Lock;

pub struct TTASLock {
    r: Arc<AtomicBool>,
}

impl Clone for TTASLock {
    fn clone(&self) -> Self {
        Self {
            r: Arc::clone(&self.r),
        }
    }
}

impl TTASLock {
    pub fn new() -> Self {
        let r = Arc::new(AtomicBool::new(false));

        Self { r }
    }
}

impl Lock for TTASLock {
    fn lock(&self) {
        loop {
            while self.r.load(Ordering::SeqCst) {}
            if !self.r.fetch_and(true, Ordering::SeqCst) {
                return;
            }
        }
    }

    fn unlock(&self) {
        self.r.fetch_and(false, Ordering::SeqCst);
    }
}

impl Drop for TTASLock {
    fn drop(&mut self) {
        self.unlock()
    }
}
