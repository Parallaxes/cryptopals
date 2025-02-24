use std::result;

use hex;
use base64;

pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    base64::encode(&bytes)
}

pub fn fixed_xor(buffer1: &str, buffer2: &str) -> String {
    if buffer1.len() != buffer2.len() {
        panic!("Buffers must be of equal length");
    }

    let mut result = String::new();
    let bytes1 = hex::decode(buffer1).unwrap();
    let bytes2 = hex::decode(buffer2).unwrap();

    for byte in bytes1.iter().zip(bytes2.iter()) {
        result.push_str(&format!("{:02x}", byte.0 ^ byte.1));
    }

    result
}

pub fn single_byte_xor(buffer: &str) {
    let keys = (0..=255).filter(|&c| (c as u8).is_ascii_alphanumeric()).collect::<Vec<u8>>();
    for key in keys {
        let result = buffer.bytes().map(|byte| byte ^ key).collect::<Vec<u8>>();
        let result = String::from_utf8(result).unwrap();
        let entropy = calculate_entropy(&result);
        println!("Key: {:02x}, Result: {}, Entropy: {}", key, result, entropy);
    }
}

fn calculate_entropy(buffer: &str) -> f64 {
    let mut entropy = 0.0;
    let mut frequency = std::collections::HashMap::new();
    let length = buffer.len() as f64;

    for byte in buffer.bytes() {
        let count = frequency.entry(byte).or_insert(0);
        *count += 1;
    }

    for count in frequency.values() {
        let probability = *count as f64 / length;
        entropy -= probability * probability.log2();
    }

    entropy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = hex_to_base64(hex);
        assert_eq!(base64, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn test_fixed_xor() {
        let buffer1 = "1c0111001f010100061a024b53535009181c";
        let buffer2 = "686974207468652062756c6c277320657965";
        let result = fixed_xor(buffer1, buffer2);
        assert_eq!(result, "746865206b696420646f6e277420706c6179");
    }
}