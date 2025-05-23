use serialize::from_hex;

pub trait Xor {
    fn fixed_xor(self, key: &str) -> Vec<u8>;
    fn rep_key_xor(self, key: &str) -> Vec<u8>;
}

impl Xor for &str {
    fn fixed_xor(self, key: &str) -> Vec<u8> {
        let buf1 = from_hex(self).unwrap();
        let buf2 = from_hex(key).unwrap();
        buf1.iter().zip(buf2.iter()).map(|(a, b)| a ^ b).collect()
    }

    fn rep_key_xor(self, key: &str) -> Vec<u8> {
        self.as_bytes()
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key.as_bytes()[i % key.len()])
            .collect()
    }
}
