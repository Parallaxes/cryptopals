use serialize::from_hex;

pub trait Xor {
    fn fixed_xor(self, input: &str) -> Vec<u8>;
    // fn brute_xor_char(self);
}

impl Xor for &str {
    fn fixed_xor(self, input: &str) -> Vec<u8> {
        let buf1 = from_hex(self).unwrap();
        let buf2 = from_hex(input).unwrap();
        buf1.iter().zip(buf2.iter()).map(|(a, b)| a ^ b).collect()
    }
}

// Attempted entropy approach to Set 01 Challenge 03
// fn char_entropy(input: &str) -> f32 {
//     let mut char_freq: HashMap<char, f32> = HashMap::new();
//     let mut sum: f32 = 0.0;

//     for char in input.chars() {
//         *char_freq.entry(char).or_insert(0.0) += 1.0;
//     }
//     let len = char_freq.len() as f32;
//     for cnt in char_freq.values_mut() {
//         let p = *cnt / len;
//         *cnt = -p * p.log(2.0);
//         sum += *cnt;
//     }

//     sum
// }

// pub fn brute_xor_char(input: &str) {
//     for i in 0..=127 {
//         let key = char::from_u32(i as u32).unwrap() as u8;
//         let input_bytes = from_hex(input).unwrap();
//         let buffer: Vec<u8> = input_bytes.iter().map(|b| b ^ key).collect();
//         if let Ok(s) = String::from_utf8(buffer) {
//             // if char_entropy(&s) >= 3.5 && char_entropy(&s) <= 5.0 {
//             //     println!("[{}] is likely an english word", s);
//             // }
//             println!("[{}: {}]", s, char_entropy(&s));
//         }
//     }
// }
