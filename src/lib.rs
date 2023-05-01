use rand::Rng;

pub trait LockID: Send {
    fn lock(&self, id: usize);
    fn unlock(&self, id: usize);
}

pub fn random(a: u64, b: u64) -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(a..b)
}
