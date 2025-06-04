use aes::Aes128;
use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input_string = std::fs::read_to_string("challenges/data/set02/challenge10.txt")
        .expect("Failed to read input");
    let input = input_string.as_bytes();
    let key = b"YELLOW SUBMARINE";
    let iv = [0u8; 16];
    let result = input.encrypt(key, Some(&iv), aes::Mode::CBC);

    println!("{:?}", result);
    Ok(())
}
