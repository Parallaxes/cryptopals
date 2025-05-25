pub trait Xor {
    fn fixed_xor(self, key: &[u8]) -> Vec<u8>;
    fn rep_key_xor(self, key: &[u8]) -> Vec<u8>;
}

impl Xor for &[u8] {
    fn fixed_xor(self, key: &[u8]) -> Vec<u8> {
        self.iter().zip(key.iter()).map(|(a, b)| a ^ b).collect()
    }

    fn rep_key_xor(self, key: &[u8]) -> Vec<u8> {
        self.iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect()
    }
}

impl Xor for &Vec<u8> {
    fn fixed_xor(self, key: &[u8]) -> Vec<u8> {
        self.iter().zip(key.iter()).map(|(a, b)| a ^ b).collect()
    }

    fn rep_key_xor(self, key: &[u8]) -> Vec<u8> {
        self.iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect()
    }
}