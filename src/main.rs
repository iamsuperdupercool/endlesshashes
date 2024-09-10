use std::io;
use std::io::Write;
use std::{thread, time::Duration};
use blake3;
use fastrand;

fn main() {
    // Get a lock on stdout
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut hash_content;
    let mut hash_content_u8;
    loop {
        hash_content = fastrand::i32(..);
        hash_content_u8 = hash_content.to_ne_bytes();
        let random_hash = blake3::hash(&hash_content_u8);
        write!(stdout, "{}\n", random_hash).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(5));
    }
}
