// CSPRNG
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use blake3;

fn convert(num: u128) -> [u8; 16] {
    num.to_ne_bytes()
}

fn seedling() -> u128 {
    let t1 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    sleep(Duration::from_millis(2));
    let t2 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    let seed = t2 - t1;
    seed % 100000
}

fn main() {
    let initial_seed_str = format!("{}{}{}{}{}", seedling(), seedling(), seedling(), seedling(), seedling());
    let mut seed = initial_seed_str.parse::<u128>().unwrap();
    loop {
        // This is very CPU-intensive
        let random_hash = blake3::hash(&convert(seed));
        println!("{}", random_hash);
        // Update seed using the hash of the current seed
        seed = u128::from_le_bytes(random_hash.as_bytes()[..16].try_into().unwrap());
    }
}
