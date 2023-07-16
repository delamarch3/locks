use std::cell::RefCell;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::Lock;

pub struct QNode {
    locked: bool,
    next: *mut QNode,
}

impl QNode {
    pub fn new() -> Self {
        Self {
            locked: false,
            next: ptr::null_mut(),
        }
    }
}

pub struct MCSLock {
    tail: AtomicPtr<QNode>,
}

unsafe impl Send for MCSLock {}

impl MCSLock {
    thread_local! {
        pub static LOCAL_NODE: RefCell<QNode> = RefCell::new(QNode::new());
    }

    pub fn new() -> Self {
        let tail = Box::into_raw(Box::new(QNode::new()));
        let tail = AtomicPtr::new(tail);

        Self { tail }
    }
}

impl Lock for MCSLock {
    fn lock(&self) {
        let q_node = MCSLock::LOCAL_NODE.with(|n| {
            // let pred = self.tail.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n|)
        });
    }

    fn unlock(&self) {
        todo!()
    }
}
