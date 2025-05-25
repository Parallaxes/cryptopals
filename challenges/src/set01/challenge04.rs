use super::challenge03::{break_single_xor, calculate_score};
use serialize::from_hex;
use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 04: ");

    let input =
        fs::read_to_string("challenges/data/set01/challenge04.txt").expect("Failed to read file");
    let result = brute(input).trim().to_owned();
    let expected = "Now that the party is jumping";

    if result == expected {
        println!("Detect single-character XOR was successful!");
        Ok(())
    } else {
        Err("Detect single-char XOR failed!".into())
    }
}

fn brute(input: String) -> String {
    let mut scores: Vec<(Vec<u8>, f32)> = Vec::new();

    for line in input.lines() {
        let bytes = from_hex(line).unwrap();
        let key = break_single_xor(&bytes);
        let decoded: Vec<u8> = bytes.iter().map(|byte| byte ^ key).collect();
        let score = calculate_score(&decoded);
        scores.push((decoded, score));
    }

    scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    String::from_utf8(scores[0].0.clone()).unwrap()
}
