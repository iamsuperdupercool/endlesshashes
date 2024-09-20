use std::io;
use std::io::Write;
use getrandom;
use blake3;
use hex;
//  write!(stdout, "{}\n", random_hash).unwrap();
//  stdout.flush().unwrap();
//  thread::sleep(Duration::from_millis(5));
//  2147483646
//  getrandom::getrandom(&mut rbuf).unwrap();
//  hex::encode(hasher.finalize());
fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut hasher = blake3::Hasher::new();
    let mut value: [u8; 32] = [0u8; 32];
    let mut constant: [u8; 64] = [0u8; 64];
    let mut counter: i32 = 0;
    getrandom::getrandom(&mut value).unwrap();
    getrandom::getrandom(&mut constant).unwrap();
    loop {
        if counter > 2147483646 {
            getrandom::getrandom(&mut value).unwrap();
            getrandom::getrandom(&mut constant).unwrap();
            counter = 0
        } else {
            counter += 1
        }
        hasher.update(&constant);
        hasher.update(&value);
        hasher.update(&counter.to_ne_bytes());
        hasher.update(blake3::hash(&value).as_bytes());
        value = *hasher.finalize().as_bytes();
        hasher.reset();
        write!(stdout, "{}\n", hex::encode(value)).unwrap();
        stdout.flush().unwrap();
    }
}
