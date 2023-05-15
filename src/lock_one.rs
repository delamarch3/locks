use crate::{Lock, ThreadIds};

pub struct LockOne {
    flag: *mut [bool; 2],

    thread_ids: ThreadIds,
}

unsafe impl Send for LockOne {}

impl Clone for LockOne {
    fn clone(&self) -> Self {
        Self {
            flag: self.flag.clone(),

            thread_ids: self.thread_ids.clone(),
        }
    }
}

impl LockOne {
    pub fn new() -> Self {
        let flag = Box::into_raw(Box::new([false, false]));
        let thread_ids = ThreadIds::new();

        Self { flag, thread_ids }
    }
}

impl Lock for LockOne {
    fn lock(&self) {
        let id = self.thread_ids.get_id();

        let j = 1 - id;

        unsafe {
            (*self.flag)[id] = true;
            while (*self.flag)[j] == true {}
        }
    }

    fn unlock(&self) {
        let id = self.thread_ids.get_id();

        unsafe {
            (*self.flag)[id] = false;
        }
    }
}
