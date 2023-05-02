use std::{thread, time::Duration};

use locks::{random, LockID};

struct PetersonLock {
    victim: *mut usize,
    flag: *mut [bool; 2],
}

unsafe impl Send for PetersonLock {}

impl Clone for PetersonLock {
    fn clone(&self) -> Self {
        Self {
            victim: self.victim.clone(),
            flag: self.flag.clone(),
        }
    }
}

impl PetersonLock {
    pub fn new() -> Self {
        let victim = Box::into_raw(Box::new(0));
        let flag = Box::into_raw(Box::new([false, false]));

        Self { victim, flag }
    }
}

impl LockID for PetersonLock {
    fn lock(&self, id: usize) {
        let j = 1 - id;
        unsafe {
            (*self.flag)[id] = true; // Declare interest
            (*self.victim) = id; // Give way

            while (*self.flag)[j] && (*self.victim) == id {}
        }
    }

    fn unlock(&self, _id: usize) {}
}

fn main() {
    let lock = PetersonLock::new();

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
