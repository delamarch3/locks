use crate::{Lock, ThreadIds};

pub struct LockTwo {
    victim: *mut usize,

    thread_ids: ThreadIds,
}

unsafe impl Send for LockTwo {}

impl Clone for LockTwo {
    fn clone(&self) -> Self {
        Self {
            victim: self.victim.clone(),

            thread_ids: self.thread_ids.clone(),
        }
    }
}

impl LockTwo {
    pub fn new() -> Self {
        let victim = Box::into_raw(Box::new(0));
        let thread_ids = ThreadIds::new();

        Self { victim, thread_ids }
    }
}

impl Lock for LockTwo {
    fn lock(&self) {
        let id = self.thread_ids.get_id();

        unsafe {
            (*self.victim) = id;

            while (*self.victim) == id {}
        }
    }

    fn unlock(&self) {}
}
