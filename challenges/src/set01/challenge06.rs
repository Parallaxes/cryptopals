use serialize::from_base64;
use xor::Xor;
use std::fs;

use super::challenge03::{break_single_xor, calculate_score};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 06: ");
    
    let input = fs::read_to_string("challenges/data/set01/challenge06.txt")
        .expect("Failed to read file")
        .lines()
        .collect::<String>()
        .trim()
        .to_string();

    let input = from_base64(&input);
    let key = break_rep_xor(&input);
    // let result = Xor::rep_key_xor(&input, &key);

    let expected = b"Terminator X: Bring the noise";
    if key == expected {
        println!("Break repeating-key XOR was successful!");
        Ok(())
    } else {
        Err("Break repeating-key XOR failed!".into())
    }
}

fn hamming_distance(b1: &[u8], b2: &[u8]) -> Option<u32> {
    if b1.len() != b2.len() {
        return None;
    }

    Some(b1.iter().zip(b2).map(|(x, y)| (x ^ y).count_ones()).sum())
}

#[test]
fn test_hamming() {
    let b1 = "this is a test".as_bytes();
    let b2 = "wokka wokka!!!".as_bytes();
    if let Some(result) = hamming_distance(b1, b2) {
        assert_eq!(result, 37);
    }
}

fn normalize_hamming(input: &[u8], keysize: usize) -> f32 {
    let chunks: Vec<&[u8]> = input.chunks(keysize).take(4).collect();
    let mut dist = 0f32;
    for i in 0..4 {
        for j in i..4 { 
            dist += hamming_distance(chunks[i], chunks[j]).unwrap() as f32;
        }
    }

    dist / keysize as f32
}

fn find_keysizes(input: &[u8]) -> Vec<usize> {
    let mut dists: Vec<(usize, u32)> = (2..=40)
        .map(|keysize| {
            (keysize, (100f32 * normalize_hamming(input, keysize)) as u32)
        }).collect();
    
    dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    dists.iter().take(3).map(|x| x.0).collect()
}


fn transpose(input: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let mut transposed: Vec<Vec<u8>> = (0..keysize).map(|_| Vec::new()).collect();
    for block in input.chunks(keysize) {
        for (&u, bt) in block.iter().zip(transposed.iter_mut()) {
            bt.push(u);
        }
    }

    transposed
}

fn break_rep_xor_ks(input: &[u8], keysize: usize) -> Vec<u8> {
    transpose(input, keysize)
        .iter()
        .map(|b| break_single_xor(b))
        .collect::<Vec<u8>>()
}

fn break_rep_xor(input: &[u8]) -> Vec<u8> {
    find_keysizes(input)
        .iter()
        .map(|&keysize| break_rep_xor_ks(input, keysize))
        .min_by_key(|key| calculate_score(&input.fixed_xor(key)) as u32)
        .unwrap()
}