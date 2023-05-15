use crate::{Lock, ThreadIds};

#[derive(Clone)]
pub struct FilterLock {
    level: *mut Vec<usize>,
    victim: *mut Vec<usize>,
    n: usize,

    thread_ids: ThreadIds,
}

unsafe impl Send for FilterLock {}

impl FilterLock {
    pub fn new(n: usize) -> Self {
        let level = Box::into_raw(Box::new(vec![0; n]));
        let victim = Box::into_raw(Box::new(vec![0; n]));
        let thread_ids = ThreadIds::new();

        Self {
            level,
            victim,
            n,
            thread_ids,
        }
    }
}

impl Lock for FilterLock {
    fn lock(&self) {
        let me = self.thread_ids.get_id();

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
        let me = self.thread_ids.get_id();

        unsafe {
            (*self.level)[me] = 0;
        }
    }
}
