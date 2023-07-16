pub mod bakery;
pub mod filter;
pub mod lock_one;
pub mod lock_two;
pub mod peterson;
pub mod practical;

use std::{
    collections::HashMap,
    sync::mpsc,
    thread::{self, ThreadId},
};

use rand::Rng;

pub trait LockID: Send {
    fn lock(&self, id: usize);
    fn unlock(&self, id: usize);
}

pub trait Lock: Send {
    fn lock(&self);
    fn unlock(&self);
}

pub fn random(a: u64, b: u64) -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(a..b)
}

pub enum ThreadIDMessage {
    Get {
        tid: ThreadId,
        tx: oneshot::Sender<usize>,
    },
}

struct ThreadIdsActor {
    rx: mpsc::Receiver<ThreadIDMessage>,
    threads: HashMap<ThreadId, usize>,
}

impl ThreadIdsActor {
    pub fn new(rx: mpsc::Receiver<ThreadIDMessage>) -> Self {
        Self {
            rx,
            threads: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, event: ThreadIDMessage) {
        match event {
            ThreadIDMessage::Get { tid, tx } => {
                // Cheack if tid exists in map, otherwise insert a new one which will be max_id + 1
                let id = match self.threads.get(&tid) {
                    Some(id) => *id,
                    None => {
                        let next_id = match self.threads.values().max() {
                            Some(id) => id + 1,
                            None => 0,
                        };

                        self.threads.insert(tid, next_id);

                        next_id
                    }
                };

                if let Err(e) = tx.send(id) {
                    eprintln!("ERROR: Could not send id - {e}")
                }
            }
        }
    }

    pub fn run(&mut self) {
        while let Ok(event) = self.rx.recv() {
            self.handle_event(event);
        }
    }
}

pub struct ThreadIds {
    tx: mpsc::Sender<ThreadIDMessage>,
}

impl Clone for ThreadIds {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

unsafe impl Send for ThreadIds {}

impl ThreadIds {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        let mut tida = ThreadIdsActor::new(rx);

        thread::spawn(move || tida.run());

        Self { tx }
    }

    pub fn get_id(&self) -> usize {
        let (tx, rx) = oneshot::channel();

        let tid = thread::current().id();
        if let Err(e) = self.tx.send(ThreadIDMessage::Get { tid, tx }) {
            eprintln!("ERROR: Could not register {tid:?} - {e}")
        }

        match rx.recv() {
            Ok(id) => id,
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
