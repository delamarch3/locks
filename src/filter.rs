use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::Lock;

pub static THREAD_IDS: AtomicUsize = AtomicUsize::new(0);
thread_local! {
    static THREAD_ID: Cell<usize> = Cell::new(THREAD_IDS.fetch_add(1, Ordering::Relaxed));
}

#[derive(Clone)]
pub struct FilterLock {
    level: *mut Vec<usize>,
    victim: *mut Vec<usize>,
    n: usize,
}

unsafe impl Send for FilterLock {}

impl FilterLock {
    pub fn new(n: usize) -> Self {
        let level = Box::into_raw(Box::new(vec![0; n]));
        let victim = Box::into_raw(Box::new(vec![0; n]));

        Self { level, victim, n }
    }
}

impl Lock for FilterLock {
    fn lock(&self) {
        let me = THREAD_ID.get();

        unsafe {
            // When i = n - 1, we can enter critical section
            for i in 1..self.n {
                (*self.level)[me] = i;
                (*self.victim)[i] = me;

                // When k == id, we can enter next level
                // Whilst the kth level is greater than or equal to i
                // And this levels victim is us
                // Block
                for k in 0..self.n {
                    while (k != me) && ((*self.level)[k] >= i && (*self.victim)[i] == me) {}
                }
            }
        }
    }

    fn unlock(&self) {
        let me = THREAD_ID.get();

        unsafe {
            (*self.level)[me] = 0;
        }
    }
}
