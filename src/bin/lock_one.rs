use std::sync::Arc;
use std::thread;
use std::time::Duration;

use locks::LockID as LockOneT;

struct LockOne {
    flag: Arc<*mut [bool; 2]>,
}

unsafe impl Send for LockOne {}

impl Clone for LockOne {
    fn clone(&self) -> Self {
        Self {
            flag: self.flag.clone(),
        }
    }
}

impl LockOne {
    pub fn new() -> Self {
        let flag = Box::into_raw(Box::new([false, false]));

        Self {
            flag: Arc::new(flag),
        }
    }
}

impl LockOneT for LockOne {
    fn lock(&self, id: usize) {
        let j = 1 - id;

        unsafe {
            (*(*self.flag))[id] = true;
            while (*(*self.flag))[j] == true {}
        }
    }

    fn unlock(&self, id: usize) {
        unsafe {
            (*(*self.flag))[id] = false;
        }
    }
}

fn main() {
    let lock = LockOne::new();

    let lock_a = lock.clone();
    let jh1 = thread::spawn(move || loop {
        let id = 0;
        lock_a.lock(id);

        eprintln!("Acquired lock for {id}");
        thread::sleep(Duration::from_secs(2));

        lock_a.unlock(id);
    });

    let lock_b = lock.clone();
    let jh2 = thread::spawn(move || {
        // Both threads can access the inner buffer at the same time and set both
        // values to true, causing a deadlock. Having one thread run before the other
        // mitigates that
        thread::sleep(Duration::from_secs(1));

        loop {
            let id = 1;
            lock_b.lock(id);

            eprintln!("Acquired lock for {id}");
            thread::sleep(Duration::from_secs(3));

            lock_b.unlock(id);
        }
    });

    let _ = jh1.join();
    let _ = jh2.join();
}
