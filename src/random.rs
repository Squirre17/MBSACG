use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;

use rand::Rng;
use rand::{rngs::StdRng,SeedableRng};
use lazy_static::lazy_static;


// static mut RNG: Option<StdRng> = None;
lazy_static! {
    static ref RNG_MTX: Mutex<Option<StdRng>> = Mutex::new(None);
}

pub fn rand_set_seed() {

    let timestamp = SystemTime::now()
                               .duration_since(UNIX_EPOCH)
                               .unwrap()
                               .as_nanos() as u64;
    let pid = std::process::id();

    let seed = timestamp ^ (pid as u64);

    *RNG_MTX.lock().unwrap() = Some(StdRng::seed_from_u64(seed));

}

#[warn(non_snake_case)]
pub fn RU32() -> u32 {
    let mut guard = RNG_MTX.lock().unwrap();
    let rng = guard.as_mut().unwrap();
    rng.gen()
}

#[warn(non_snake_case)]
pub fn RU64() -> u64 {
    let mut guard = RNG_MTX.lock().unwrap();
    let rng = guard.as_mut().unwrap();
    rng.gen()
}