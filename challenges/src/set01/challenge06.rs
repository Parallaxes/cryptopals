use serialize::string_from_vec;
use std::fs;

use super::challenge03::break_single_xor;

static KEYSIZE: i32 = 40;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input =
        fs::read_to_string("challenges/data/set01/challenge06.txt").expect("Failed to read file");
    let candidates = find_candidates(input.clone());
    println!("{}", break_rep_xor(input, candidates));
    Ok(())
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

fn find_candidates(input: String) -> (i32, i32, i32) {
    let input_bytes: Vec<u8> = input
        .bytes()
        .filter(|&b| b != b'\n' && b != b'\r')
        .collect();

    let mut candidates: Vec<(i32, u32)> = Vec::new();

    for i in 1..=KEYSIZE {
        let i = i as usize;
        if input_bytes.len() < 2 * i { continue; }
        if let Some(dist) = hamming_distance(&input_bytes[0..i], &input_bytes[i..2*i]) {
            candidates.push((i as i32, dist));
        }
    }

    candidates.sort_by_key(|i| i.1);

    if candidates.len() >= 3 {
        (candidates[0].0, candidates[1].0, candidates[2].0)
    } else {
        (0, 0, 0)
    }
}

fn break_rep_xor(input: String, candidates: (i32, i32, i32)) -> String {
    let candidate_sizes = [candidates.0, candidates.1, candidates.2];
    for i in 0..3 {
        let keysize = candidate_sizes[i] as usize;
        if keysize == 0 {
            eprintln!("Skipping candidate with zero keysize");
            continue;
        }

        let mut blocks: Vec<Vec<u8>> = input
            .as_bytes()
            .chunks(keysize)
            .map(|chunk| chunk.to_vec())
            .collect();

        transpose(&mut blocks);

        for block in blocks {
            println!("{:?}", break_single_xor(std::str::from_utf8(&block).unwrap())); // TODO: FIX
        }
    }
    "Ok".to_string()
}

fn transpose(blocks: &mut Vec<Vec<u8>>) {
    if blocks.is_empty() || blocks[0].is_empty() {
        return;
    }

    let num_blocks = blocks.len();
    let block_size = blocks[0].len();

    let mut transposed: Vec<Vec<u8>> = vec![Vec::with_capacity(num_blocks); block_size];

    for block in blocks.iter() {
        for (i, &byte) in block.iter().enumerate() {
            transposed[i].push(byte);
        }
    }

    *blocks = transposed;
}
