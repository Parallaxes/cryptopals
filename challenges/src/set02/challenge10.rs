use aes::Aes128Suite;
use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("challenges/data/set02/challenge10.txt")
        .expect("Failed to read input")
        .as_bytes();

    Ok(())
}
