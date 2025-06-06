use templar::Oracle11;
use std::collections::HashSet;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 02 Challenge 11: ");

    let input = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    for _ in 0..10 {
        let oracle = Oracle11::new(input);
        let result = detect_ecb(&oracle.ciphertext);
        if result != oracle.mode {
            return Err("An ECB/CBC detection oracle failed!".into());
        }
    }
    
    println!("An ECB/CBC detection oracle was successful!");
    Ok(())
    
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