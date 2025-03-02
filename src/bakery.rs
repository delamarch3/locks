use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::Lock;

pub static THREAD_IDS: AtomicUsize = AtomicUsize::new(0);
thread_local! {
    static THREAD_ID: Cell<usize> = Cell::new(THREAD_IDS.fetch_add(1, Ordering::Relaxed));
}

#[derive(Clone)]
pub struct Bakery {
    flag: *mut Vec<bool>,
    label: *mut Vec<usize>,
    n: usize,
}

impl Bakery {
    pub fn new(n: usize) -> Self {
        let flag = Box::into_raw(Box::new(vec![false; n]));
        let label = Box::into_raw(Box::new(vec![0; n]));

        Self { flag, label, n }
    }
}

unsafe impl Send for Bakery {}

impl Lock for Bakery {
    fn lock(&self) {
        let id = THREAD_ID.get();

        unsafe {
            // Doorway section completes in a bounded numebr of steps (bounded wait-free)
            {
                // Set our flag to true
                (*self.flag)[id] = true;
                // Get the max label:
                (*self.label)[id] = (*self.label).iter().max().unwrap_or_else(|| &0) + 1;
            }

            // Waiting section:

            // When k == id, we can check next label
            // Whilst the kth label is less than our label
            // Or the kth label is equal to our label, and k is less than our id
            // Block
            for k in 0..self.n {
                while (k != id)
                    && (*self.flag)[k]
                    && ((*self.label)[k] < (*self.label)[id]
                        || ((*self.label)[k] == (*self.label)[id] && k < id))
                {}
            }
        }
    }

    fn unlock(&self) {
        let id = THREAD_ID.get();

        unsafe {
            (*self.flag)[id] = false;
        }
    }
}
