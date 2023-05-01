use std::{sync::Arc, thread, time::Duration};

use locks::{random, LockID};

struct LockTwo {
    victim: Arc<*mut usize>,
}

unsafe impl Send for LockTwo {}

impl Clone for LockTwo {
    fn clone(&self) -> Self {
        Self {
            victim: self.victim.clone(),
        }
    }
}

impl LockTwo {
    pub fn new() -> Self {
        let victim = Box::into_raw(Box::new(0));

        Self {
            victim: Arc::new(victim),
        }
    }
}

impl LockID for LockTwo {
    fn lock(&self, id: usize) {
        unsafe {
            (*(*self.victim)) = id;

            while (*(*self.victim)) == id {}
        }
    }

    fn unlock(&self, id: usize) {}
}

fn main() {
    let lock = LockTwo::new();

    let lock_a = lock.clone();
    let jh1 = thread::spawn(move || loop {
        let id = 0;
        lock_a.lock(id);

        eprintln!("Acquired lock for {id}");
        thread::sleep(Duration::from_secs(random(2, 6)));

        lock_a.unlock(id);
    });

    let lock_b = lock.clone();
    let jh2 = thread::spawn(move || loop {
        let id = 1;
        lock_b.lock(id);

        eprintln!("Acquired lock for {id}");
        thread::sleep(Duration::from_secs(random(2, 6)));

        lock_b.unlock(id);
    });

    let _ = jh1.join();
    let _ = jh2.join();
}
