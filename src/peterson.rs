use crate::{Lock, ThreadIds};

pub struct PetersonLock {
    victim: *mut usize,
    flag: *mut [bool; 2],

    thread_ids: ThreadIds,
}

unsafe impl Send for PetersonLock {}

impl Clone for PetersonLock {
    fn clone(&self) -> Self {
        Self {
            victim: self.victim.clone(),
            flag: self.flag.clone(),
            thread_ids: self.thread_ids.clone(),
        }
    }
}

impl PetersonLock {
    pub fn new() -> Self {
        let victim = Box::into_raw(Box::new(0));
        let flag = Box::into_raw(Box::new([false, false]));
        let thread_ids = ThreadIds::new();

        Self {
            victim,
            flag,
            thread_ids,
        }
    }
}

impl Lock for PetersonLock {
    fn lock(&self) {
        let id = self.thread_ids.get_id();

        let j = 1 - id;
        unsafe {
            (*self.flag)[id] = true; // Declare interest
            (*self.victim) = id; // Give way

            while (*self.flag)[j] && (*self.victim) == id {}
        }
    }

    fn unlock(&self) {}
}
