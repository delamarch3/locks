use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::Lock;

pub static THREAD_IDS: AtomicUsize = AtomicUsize::new(0);
thread_local! {
    static THREAD_ID: Cell<usize> = Cell::new(THREAD_IDS.fetch_add(1, Ordering::Relaxed));
}

pub struct PetersonLock {
    victim: *mut usize,
    flag: *mut [bool; 2],
}

unsafe impl Send for PetersonLock {}

impl Clone for PetersonLock {
    fn clone(&self) -> Self {
        Self {
            victim: self.victim.clone(),
            flag: self.flag.clone(),
        }
    }
}

impl PetersonLock {
    pub fn new() -> Self {
        let victim = Box::into_raw(Box::new(0));
        let flag = Box::into_raw(Box::new([false, false]));

        Self { victim, flag }
    }
}

impl Lock for PetersonLock {
    fn lock(&self) {
        let id = THREAD_ID.get();

        let j = 1 - id;
        unsafe {
            (*self.flag)[id] = true; // Declare interest
            (*self.victim) = id; // Give way

            while (*self.flag)[j] && (*self.victim) == id {}
        }
    }

    fn unlock(&self) {}
}
