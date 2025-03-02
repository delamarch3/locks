pub mod bakery;
pub mod filter;
pub mod lock_one;
pub mod lock_two;
pub mod peterson;
pub mod practical;

// This project manually implements of thread IDs because their values are used for arithmetic in
// certain lock implementations
// https://github.com/rust-lang/rust/issues/67939 tracks thread ID values

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
