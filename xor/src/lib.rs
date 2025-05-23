use serialize::from_hex;

pub trait Xor {
    fn fixed_xor(self, input: &str) -> Vec<u8>;
}

impl Xor for &str {
    fn fixed_xor(self, input: &str) -> Vec<u8> {
        let buf1 = from_hex(self).unwrap();
        let buf2 = from_hex(input).unwrap();
        buf1.iter().zip(buf2.iter()).map(|(a, b)| a ^ b).collect()
    }
}
