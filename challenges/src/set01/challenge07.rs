use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("challenges/data/set01/challenge06.txt")
        .expect("Failed to read file")
        .lines()
        .collect::<String>()
        .trim()
        .to_string();
    
    let key = b"YELLOW SUBMARINE";
    
    Ok(())
}