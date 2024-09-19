use std::io;
use std::io::Write;
use std::{thread, time::Duration};
use getrandom;
use sha3::{Digest, Sha3_256};
use hex;
fn main() {
    // Get a lock on stdout
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut hasher = Sha3_256::new();
    let mut rbuf = [0u8; 256];
    let mut salt = [0u8; 24];
    let mut hash_counter: i32 = 0;
    let mut random_hash;
    getrandom::getrandom(&mut rbuf).unwrap();
    getrandom::getrandom(&mut salt).unwrap();
    loop {
        if hash_counter > 2147483646 {
            getrandom::getrandom(&mut rbuf).unwrap();
            getrandom::getrandom(&mut salt).unwrap();
            hash_counter = 0;
        } else {
            hash_counter += 1
        }
        hasher.update(rbuf);
        hasher.update(salt);
        hasher.update(hash_counter.to_ne_bytes());
        random_hash = hex::encode(hasher.finalize());
        rbuf = hasher.finalize_reset()
        write!(stdout, "{}\n", random_hash).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(5));
    }
}
