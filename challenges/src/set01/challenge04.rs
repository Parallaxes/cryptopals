use super::challenge03::break_single_xor;
use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 04: ");

    let input =
        fs::read_to_string("challenges/data/set01/challenge04.txt").expect("Failed to read file");
    let result = brute(input).trim().to_string();
    let expected = "Now that the party is jumping";

    if result == expected {
        println!("Detect single-character XOR was successful!");
        Ok(())
    } else {
        Err("Detect single-char XOR failed!".into())
    }
}

fn brute(input: String) -> String {
    let mut results: Vec<(String, f32)> =
        input.lines().map(|line| break_single_xor(line)).collect();

    if results.is_empty() {
        return "No valid strings found".to_string();
    }

    results.sort_by(|a: &(String, f32), b| a.1.partial_cmp(&b.1).unwrap());
    results[0].0.clone()
}
