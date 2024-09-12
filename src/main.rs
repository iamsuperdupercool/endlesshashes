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
    let mut rbuf = [0u8; 32];
    let mut random_hash;
    loop {
        getrandom::getrandom(&mut rbuf).unwrap();
        hasher.update(rbuf);
        random_hash = hex::encode(hasher.finalize_reset());
        write!(stdout, "{}\n", random_hash).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(5));
    }
}
