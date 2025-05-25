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
    let result = String::from_utf8(
        from_hex(input)
            .unwrap()
            .iter()
            .map(|&b| b ^ break_single_xor(&from_hex(input).unwrap()))
            .collect(),
    )
    .unwrap();
    let expected = "Cooking MC's like a pound of bacon";

    if result == expected {
        println!("Single-byte XOR cipher was successful!");
        Ok(())
    } else {
        Err("Single-byte XOR cipher failed!".into())
    }
}

fn calculate_freqs(input: &[u8]) -> HashMap<u8, f32> {
    let mut char_freqs: HashMap<u8, f32> = HashMap::new();

    for byte in input {
        *char_freqs.entry(*byte).or_insert(0.0) += 1.0;
    }

    let len: f32 = char_freqs.values().sum();
    for freq in char_freqs.values_mut() {
        *freq /= len;
    }

    char_freqs
}

pub fn calculate_score(input: &[u8]) -> f32 {
    let char_freqs = calculate_freqs(input);

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

pub fn break_single_xor(input: &[u8]) -> u8 {
    let mut scores: Vec<(u8, f32)> = Vec::new();

    for key in 0..=255 {
        let buffer: Vec<u8> = input.iter().map(|b| b ^ key).collect();
        scores.push((key, calculate_score(&buffer)))
    }

    scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    scores[0].0
}
