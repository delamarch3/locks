use std::{thread, time::Duration};

use locks::{lock_two::LockTwo, random, Lock};

fn main() {
    let lock = LockTwo::new();

    let lock_a = lock.clone();
    let jh1 = thread::spawn(move || loop {
        let id = 0;
        lock_a.lock();

        eprintln!("Acquired lock for {id}");
        thread::sleep(Duration::from_secs(random(2, 6)));

        lock_a.unlock();
    });

    let lock_b = lock.clone();
    let jh2 = thread::spawn(move || loop {
        let id = 1;
        lock_b.lock();

        eprintln!("Acquired lock for {id}");
        thread::sleep(Duration::from_secs(random(2, 6)));

        lock_b.unlock();
    });

    let _ = jh1.join();
    let _ = jh2.join();
}
