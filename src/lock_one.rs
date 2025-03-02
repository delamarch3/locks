use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::Lock;

pub static THREAD_IDS: AtomicUsize = AtomicUsize::new(0);
thread_local! {
    static THREAD_ID: Cell<usize> = Cell::new(THREAD_IDS.fetch_add(1, Ordering::Relaxed));
}

pub struct LockOne {
    flag: *mut [bool; 2],
}

unsafe impl Send for LockOne {}

impl Clone for LockOne {
    fn clone(&self) -> Self {
        Self {
            flag: self.flag.clone(),
        }
    }
}

impl LockOne {
    pub fn new() -> Self {
        let flag = Box::into_raw(Box::new([false, false]));

        Self { flag }
    }
}

impl Lock for LockOne {
    fn lock(&self) {
        let id = THREAD_ID.get();

        let j = 1 - id;

        unsafe {
            (*self.flag)[id] = true;
            while (*self.flag)[j] == true {}
        }
    }

    fn unlock(&self) {
        let id = THREAD_ID.get();

        unsafe {
            (*self.flag)[id] = false;
        }
    }
}
