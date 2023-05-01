use locks::LockID;

struct FilterLock {
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

impl LockID for FilterLock {
    fn lock(&self, me: usize) {
        unsafe {
            for i in 1..self.n {
                (*self.level)[me] = i;
                (*self.victim)[i] = me;

                for k in 0..self.n {
                    if k != me {
                        while (*self.level)[k] >= i && (*self.victim)[i] == me {}
                        break;
                    }
                }
            }
        }
    }

    fn unlock(&self, id: usize) {
        todo!()
    }
}

fn main() {
    let lock = FilterLock::new(5);
}
