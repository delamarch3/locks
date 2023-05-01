use std::{thread, time::Duration};

use locks::{random, LockID};

#[derive(Clone)]
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

    fn unlock(&self, me: usize) {
        unsafe {
            (*self.level)[me] = 0;
        }
    }
}

fn main() {
    let lock = FilterLock::new(3);

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

    let lock_c = lock.clone();
    let jh3 = thread::spawn(move || loop {
        let id = 2;
        lock_c.lock(id);

        eprintln!("Acquired lock for {id}");
        thread::sleep(Duration::from_secs(random(2, 6)));

        lock_c.unlock(id);
    });

    let _ = jh1.join();
    let _ = jh2.join();
    let _ = jh3.join();
}
