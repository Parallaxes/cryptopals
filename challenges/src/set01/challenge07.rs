use std::fs;
use aes::Decrypt;
use serialize::from_base64;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("challenges/data/set01/challenge06.txt")
        .expect("Failed to read file")
        .lines()
        .collect::<String>(); 

    let key = b"YELLOW SUBMARINE";
    if from_base64(&input).len() % 16 != 0 {
        println!("Panic");
    }
    let result = from_base64(&input).as_slice().decrypt(key, None, "ECB");
    println!("{:?}", result);
    Ok(())
}
