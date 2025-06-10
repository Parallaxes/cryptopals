use rand::Rng;
use aes::Aes128;
use serialize::from_base64_file;

pub struct Oracle11 {
    pub ciphertext: Vec<u8>,
    pub mode: aes::Mode, 
}

impl Oracle11 {
    pub fn new(input: &[u8]) -> Self {
        let mut rng = rand::rng();
        let mut key = [0u8; 16];

        for byte in key.iter_mut() {
            *byte = rng.random();
        }

        let prefix_len = rng.random_range(5..=10);
        let suffix_len = rng.random_range(5..=10);
        let mut prefix = vec![0u8; prefix_len];
        rng.fill(&mut prefix[..]);
        let mut suffix = vec![0u8; suffix_len];
        rng.fill(&mut suffix[..]);

        let mut buf: Vec<u8> = Vec::with_capacity(input.len() + prefix_len + suffix_len);
        buf.extend_from_slice(&prefix);
        buf.extend_from_slice(&input);
        buf.extend_from_slice(&suffix);
        let buf = buf.pad(16);
        
        if rng.random_range(1..=2) % 2 == 0 {
            let result = buf.encrypt(&key, None, aes::Mode::ECB).unwrap();
            return Oracle11 { ciphertext: result, mode: aes::Mode::ECB };
        } else {
            let mut iv = vec![0u8; 16];
            rng.fill(&mut iv[..]);
            let result = buf.encrypt(&key, Some(&iv), aes::Mode::CBC).unwrap();
            return Oracle11 { ciphertext: result, mode: aes::Mode::CBC };
        }
    }
}

pub struct Oracle12 {
    ciphertext: Vec<u8>,
    key: Vec<u8>,
}

impl Oracle12 {
    pub fn new(input: &[u8], key: &[u8]) -> Self {
        
    }

    pub fn gen_key() -> Vec<u8> {
        let mut rng = rand::rng();
        let mut key = [0u8; 16];

        for byte in key.iter_mut() {
            *byte = rng.random();
        }

        key.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}