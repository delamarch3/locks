use crate::{Lock, ThreadIds};

#[derive(Clone)]
pub struct Bakery {
    flag: *mut Vec<bool>,
    label: *mut Vec<usize>,
    n: usize,

    thread_ids: ThreadIds,
}

impl Bakery {
    pub fn new(n: usize) -> Self {
        let flag = Box::into_raw(Box::new(vec![false; n]));
        let label = Box::into_raw(Box::new(vec![0; n]));
        let thread_ids = ThreadIds::new();

        Self {
            flag,
            label,
            n,
            thread_ids,
        }
    }
}

unsafe impl Send for Bakery {}

impl Lock for Bakery {
    fn lock(&self) {
        let id = self.thread_ids.get_id();

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
        let id = self.thread_ids.get_id();

        unsafe {
            (*self.flag)[id] = false;
        }
    }
}
