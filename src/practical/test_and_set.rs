use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::Lock;

pub struct TASLock {
    r: Arc<AtomicBool>,
}

impl Clone for TASLock {
    fn clone(&self) -> Self {
        Self {
            r: Arc::clone(&self.r),
        }
    }
}

impl TASLock {
    pub fn new() -> Self {
        let r = Arc::new(AtomicBool::new(false));

        Self { r }
    }
}

impl Lock for TASLock {
    fn lock(&self) {
        while self.r.fetch_and(true, Ordering::SeqCst) {}
    }

    fn unlock(&self) {
        self.r.fetch_and(false, Ordering::SeqCst);
    }
}

impl Drop for TASLock {
    fn drop(&mut self) {
        self.unlock()
    }
}
