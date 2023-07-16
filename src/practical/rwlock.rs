use std::sync::{Arc, Condvar, Mutex};

pub struct RwLockState {
    writer: bool,
    read_acquires: u64,
    read_releases: u64,
}

#[derive(Clone)]
pub struct RwLock {
    inner: Arc<(Mutex<RwLockState>, Condvar)>,
}

unsafe impl Send for RwLock {}

pub struct ReadGuard<'a> {
    lock_ref: &'a RwLock,
}

impl Drop for ReadGuard<'_> {
    fn drop(&mut self) {
        let mut state = self.lock_ref.inner.0.lock().expect("Poisoned mutex");
        state.read_releases += 1;

        if state.read_releases == state.read_acquires {
            self.lock_ref.inner.1.notify_all();
        }
    }
}

pub struct WriteGuard<'a> {
    lock_ref: &'a RwLock,
}

impl Drop for WriteGuard<'_> {
    fn drop(&mut self) {
        let mut state = self.lock_ref.inner.0.lock().expect("Poisoned mutex");
        state.writer = false;

        self.lock_ref.inner.1.notify_all();
    }
}

impl RwLock {
    pub fn new() -> Self {
        let condition = Condvar::new();
        let writer = false;
        let read_acquires = 0;
        let read_releases = 0;

        let state = Mutex::new(RwLockState {
            writer,
            read_acquires,
            read_releases,
        });

        let inner = Arc::new((state, condition));

        Self { inner }
    }

    pub fn read<'a>(&'a self) -> ReadGuard<'a> {
        let mut state = self.inner.0.lock().expect("Poisoned mutex");

        while state.writer {
            state = self.inner.1.wait(state).expect("Poisoned mutex");
        }

        state.read_acquires += 1;

        drop(state);
        ReadGuard { lock_ref: self }
    }

    pub fn write<'a>(&'a self) -> WriteGuard<'a> {
        let mut state = self.inner.0.lock().expect("Poisoned mutex");

        while state.writer {
            state = self.inner.1.wait(state).expect("Poisoned mutex");
        }

        state.writer = true;
        while state.read_acquires != state.read_releases {
            state = self.inner.1.wait(state).expect("Poisoned mutex");
        }

        drop(state);
        WriteGuard { lock_ref: self }
    }

    pub fn write_unlock(&self) {
        let mut state = self.inner.0.lock().expect("Poisoned mutex");
        state.writer = false;
        self.inner.1.notify_all();
    }
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use crate::practical::rwlock::RwLock;

    #[test]
    fn test_rwlock() {
        let lock_a = RwLock::new();

        let lock_b = lock_a.clone();
        let jh = thread::spawn(move || {
            let w = lock_b.write();
            thread::sleep(Duration::from_secs(1));
            drop(w);
        });

        {
            let _ra = lock_a.read();
            let _rb = lock_a.read();
        }

        jh.join().unwrap();
    }
}
