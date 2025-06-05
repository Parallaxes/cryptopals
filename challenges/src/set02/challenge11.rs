use aes::Aes128;
use rand::Rng;
use templar::Oracle1;
use std::collections::HashSet;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 02 Challenge 11: ");

    let input = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let oracle = Oracle1::new(input);
    let result = detect_ecb(&oracle.ciphertext);
    
    // TODO: *Major* fix
    if result == oracle.mode {
        println!("An ECB/CBC detection oracle was successful!");
        Ok(())
    } else {
        Err("An ECB/CBC detection oracle".into())
    }
}

fn detect_ecb(input: &[u8]) -> aes::Mode {
    let mut seen: HashSet<Vec<u8>> = HashSet::new();
    for chunk in input.chunks(16) {
        if !seen.insert(chunk.to_vec()) {
            return aes::Mode::ECB;
        }
    }

    aes::Mode::CBC
}