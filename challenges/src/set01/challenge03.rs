use serialize::from_hex;
use std::collections::HashMap;

static EXPECTED_FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 12.17), // Whitespace
    (b'.', 6.57),  // Others
    (b'a', 6.09),
    (b'b', 1.05),
    (b'c', 2.84),
    (b'd', 2.92),
    (b'e', 11.36),
    (b'f', 1.79),
    (b'g', 1.38),
    (b'h', 3.41),
    (b'i', 5.44),
    (b'j', 0.24),
    (b'k', 0.41),
    (b'l', 2.92),
    (b'm', 2.76),
    (b'n', 5.44),
    (b'o', 6.00),
    (b'p', 1.95),
    (b'q', 0.24),
    (b'r', 4.95),
    (b's', 5.68),
    (b't', 8.03),
    (b'u', 2.43),
    (b'v', 0.97),
    (b'w', 1.38),
    (b'x', 0.24),
    (b'y', 1.30),
    (b'z', 0.03),
];

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 03: ");

    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let result = break_single_xor(input).0;
    let expected = "Cooking MC's like a pound of bacon";

    if result == expected {
        println!("Single-byte XOR cipher was successful!");
        Ok(())
    } else {
        Err("Single-byte XOR cipher failed!".into())
    }
}

fn calculate_freqs(input: &str) -> HashMap<u8, f32> {
    let mut char_freq: HashMap<u8, f32> = HashMap::new();

    for byte in input.bytes() {
        *char_freq.entry(byte).or_insert(0.0) += 1.0;
    }

    let len: f32 = char_freq.values().sum();
    for cnt in char_freq.values_mut() {
        *cnt /= len;
    }

    char_freq
}

fn calculate_score(char_freqs: HashMap<u8, f32>) -> f32 {
    let mut score = 0.0;

    for (byte, cnt) in &char_freqs {
        if byte.is_ascii_alphabetic() || *byte == b' ' || *byte == b'.' {
            if let Some(expected) = get_expected(*byte) {
                score += (expected - cnt).abs();
            }
        } else {
            score += 100.0; // Penalize unrecognized chars
        }
    }

    score
}

fn get_expected(byte: u8) -> Option<f32> {
    for (b, freq) in EXPECTED_FREQUENCIES.iter() {
        if *b == byte {
            return Some(*freq);
        }
    }

    None
}

pub fn break_single_xor(input: &str) -> (String, f32) {
    let mut results: Vec<(String, f32)> = Vec::new();
    for i in 0..=255 {
        let buffer: Vec<u8> = from_hex(input).unwrap().iter().map(|b| b ^ i).collect();
        if let Ok(s) = String::from_utf8(buffer) {
            let score = calculate_score(calculate_freqs(&s));
            results.push((s, score));
        }
    }

    if results.is_empty() {
        return ("".to_string(), f32::MAX);
    }

    results.sort_by(|a: &(String, f32), b| a.1.partial_cmp(&b.1).unwrap());
    results[0].clone()
}
