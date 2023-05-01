use std::thread;
use std::time::Duration;

use locks::{random, LockID};

#[derive(Clone)]
pub struct Bakery {
    flag: *mut Vec<bool>,
    label: *mut Vec<usize>,
    n: usize,
}

impl Bakery {
    pub fn new(n: usize) -> Self {
        let flag = Box::into_raw(Box::new(vec![false; n]));
        let label = Box::into_raw(Box::new(vec![0; n]));

        Self { flag, label, n }
    }
}

unsafe impl Send for Bakery {}

impl LockID for Bakery {
    fn lock(&self, id: usize) {
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

    fn unlock(&self, id: usize) {
        unsafe {
            (*self.flag)[id] = false;
        }
    }
}

fn main() {
    let lock = Bakery::new(3);

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
