use std::thread;
use std::time::Duration;

use locks::lock_one::LockOne;
use locks::Lock;

fn main() {
    let lock = LockOne::new();

    let lock_a = lock.clone();
    let jh1 = thread::spawn(move || loop {
        let id = 0;
        lock_a.lock();

        eprintln!("Acquired lock for {id}");
        thread::sleep(Duration::from_secs(2));

        lock_a.unlock();
    });

    let lock_b = lock.clone();
    let jh2 = thread::spawn(move || {
        // Both threads can access the inner buffer at the same time and set both
        // values to true, causing a deadlock. Having one thread run before the other
        // mitigates that
        thread::sleep(Duration::from_secs(1));

        loop {
            let id = 1;
            lock_b.lock();

            eprintln!("Acquired lock for {id}");
            thread::sleep(Duration::from_secs(3));

            lock_b.unlock();
        }
    });

    let _ = jh1.join();
    let _ = jh2.join();
}
