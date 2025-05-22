use std::collections::HashMap;
use serialize::from_hex;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let DIAGRAPH: HashMap<&str, (u32, f32)> = HashMap::from([
        ("th", (5532, 1.52)),
        ("he", (4657, 1.28)),
        ("in", (3429, 0.94)),
        ("er", (3420, 0.94)),
        ("an", (3005, 0.82)),
        ("re", (2465, 0.68)),
        ("nd", (2281, 0.63)),
        ("at", (2155, 0.59)),
        ("on", (2086, 0.57)),
        ("nt", (2058, 0.56)),
        ("ha", (2040, 0.56)),
        ("es", (2033, 0.56)),
        ("st", (2009, 0.55)),
        ("en", (2005, 0.55)),
        ("ed", (1942, 0.53)),
        ("to", (1904, 0.52)),
        ("it", (1822, 0.50)),
        ("ou", (1820, 0.50)),
        ("ea", (1720, 0.47)),
        ("hi", (1690, 0.46)),
        ("is", (1660, 0.46)),
        ("or", (1556, 0.43)),
        ("ti", (1231, 0.34)),
        ("as", (1211, 0.33)),
        ("te", (985, 0.27)),
        ("et", (704, 0.19)),
        ("ng", (668, 0.18)),
        ("of", (569, 0.16)),
        ("al", (341, 0.09)),
        ("de", (332, 0.09)),
        ("se", (300, 0.08)),
        ("le", (298, 0.08)),
        ("sa", (215, 0.06)),
        ("si", (186, 0.05)),
        ("ar", (157, 0.04)),
        ("ve", (148, 0.04)),
        ("ra", (137, 0.04)),
        ("ld", (64, 0.02)),
        ("ur", (60, 0.02)),
    ]);

    let LETTER_FREQS: HashMap<char, (u32, f32)> = HashMap::from([
        ('e', (21912, 12.02)),
        ('t', (16587, 9.10)),
        ('a', (14810, 8.12)),
        ('o', (14003, 7.68)),
        ('i', (13318, 7.31)),
        ('n', (12666, 6.95)),
        ('s', (11450, 6.28)),
        ('r', (10977, 6.02)),
        ('h', (10795, 5.92)),
        ('d', (7874, 4.32)),
        ('l', (7253, 3.98)),
        ('u', (5246, 2.88)),
        ('c', (4943, 2.71)),
        ('m', (4761, 2.61)),
        ('f', (4200, 2.30)),
        ('y', (3853, 2.11)),
        ('w', (3819, 2.09)),
        ('g', (3693, 2.03)),
        ('p', (3316, 1.82)),
        ('b', (2715, 1.49)),
        ('v', (2019, 1.11)),
        ('k', (1257, 0.69)),
        ('x', (315, 0.17)),
        ('q', (205, 0.11)),
        ('j', (188, 0.10)),
        ('z', (128, 0.07)),
    ]);

    let mut results: Vec<(String, f32)> = Vec::new();
    for i in 0..=1 {
        let key = i as u8;
        let input_bytes = from_hex(input).unwrap();
        let buffer: Vec<u8> = input_bytes.iter().map(|b| b ^ key).collect();
        if let Ok(s) = String::from_utf8(buffer) {
            let score = calc_score(&s, LETTER_FREQS.clone(), DIAGRAPH.clone());
            results.push((s, score));
        }
    }

    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for (s, score) in results.iter().take(100) {
        println!("[{}: {}]", s, score);
    }
    

    Ok(())
}

fn calc_char_freqs(input: &str) -> (HashMap<char, f32>, u32) {
    let mut char_freq: HashMap<char, f32> = HashMap::new();
    let input = input.trim().to_ascii_lowercase();
    let mut invalid_cnt = 0;
    
    for char in input.chars() {
        if ('a'..='z').contains(&char) {
            *char_freq.entry(char).or_insert(0.0) += 1.0;
        } else {
            println!("Invalid! [{}]", char);
            invalid_cnt += 1;
        }
    }

    let len = char_freq.values().sum::<f32>() + invalid_cnt as f32;
    for cnt in char_freq.values_mut() {
        *cnt /= len;
    }

    (char_freq, invalid_cnt)
}

fn calc_score(input: &str, letters: HashMap<char, (u32, f32)>, diagraphs: HashMap<&str, (u32, f32)>) -> f32 {
    let input = input.trim().to_ascii_lowercase();
    let char_freq = calc_char_freqs(&input);
    let mut sum = 0.0 as f32;
    
    for (char, cnt) in char_freq.0 {
        if let Some((_, expected_freq)) = letters.get(&char) {
            sum += (cnt - expected_freq).abs();
        } else {
            println!("Invalid char!");
            sum += 10000.0 * char_freq.1 as f32; // penalize unknown chars
        }
    }

    // Calculate diagraph frequencies in the input
    let mut diagraph_freq: HashMap<String, f32> = HashMap::new();
    let chars: Vec<char> = input.chars().collect();
    let total_diagraphs = if chars.len() > 1 { chars.len() - 1 } else { 0 } as f32;

    for i in 0..chars.len().saturating_sub(1) {
        let diagraph = format!("{}{}", chars[i], chars[i + 1]);
        *diagraph_freq.entry(diagraph).or_insert(0.0) += 1.0;
    }

    for cnt in diagraph_freq.values_mut() {
        *cnt /= total_diagraphs.max(1.0);
    }

    for (diagraph, cnt) in diagraph_freq {
        if let Some((_, expected_freq)) = diagraphs.get(diagraph.as_str()) {
            sum += (cnt - expected_freq).abs();
        } else {
            sum += cnt; // penalize unknown diagraphs
        }
    }

    sum
}