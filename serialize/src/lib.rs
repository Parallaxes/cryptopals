use base64::{engine::general_purpose, Engine as _};

pub trait Serialize {
    fn to_hex(self) -> String;
    fn to_base64(self) -> Option<String>;
}

impl Serialize for &str {
    fn to_hex(self) -> String {
        self.chars()
            .map(|c| format!("{:02X}", c as u8))
            .collect::<Vec<_>>()
            .join("")
    }

    fn to_base64(self) -> Option<String> {
        Some(general_purpose::STANDARD.encode(self))
    }
}

impl Serialize for &[u8] {
    fn to_hex(self) -> String {
        self.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn to_base64(self) -> Option<String> {
        Some(general_purpose::STANDARD.encode(self))
    }
}

impl Serialize for Vec<u8> {
    fn to_hex(self) -> String {
        self.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn to_base64(self) -> Option<String> {
        Some(general_purpose::STANDARD.encode(&self))
    }
}

pub fn from_hex(input: &str) -> Option<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::with_capacity(input.len() / 2);
    for chunk in input.as_bytes().chunks(2) {
        if let Ok(byte_val) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16) {
            bytes.push(byte_val);
        } else {
            return None;
        }
    }

    Some(bytes)
}

pub fn from_base64(input: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(input).unwrap()
}

use std::fs;

pub fn from_base64_file(path: &str) -> Vec<u8> {
    from_base64(
        &fs::read_to_string(path)
            .expect("Failed to read file")
            .lines()
            .collect::<String>(),
    )
}

pub fn string_from_vec(input: Vec<u8>) -> String {
    let mut result = String::new();
    for elem in input {
        result.push(elem as char);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        let input = "6D656F77203A33";
        assert_eq!(Some(vec![109, 101, 111, 119, 32, 58, 51]), from_hex(input));
    }

    #[test]
    fn test_to_base64() {
        let input = "meow :3";
        assert_eq!(Some(String::from("bWVvdyA6Mw==")), input.to_base64());
    }
}
