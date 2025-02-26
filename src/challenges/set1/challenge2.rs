// Fixed XOR
//
// Write a function that takes two equal-length buffers and produces their XOR combination.
//
// If your function works properly, then when you feed it the string:
// 1c0111001f010100061a024b53535009181c
//
// ... after hex decoding, and when XOR'd against:
// 686974207468652062756c6c277320657965
//
// ... should produce:
// 746865206b696420646f6e277420706c6179

pub fn main() {
    let input = "1c0111001f010100061a024b53535009181c";

    let decoded = hex::decode(input).unwrap();
    let xor = hex::decode("686974207468652062756c6c277320657965").unwrap();

    let result = decoded.iter().zip(xor.iter()).map(|(a, b)| a ^ b).collect::<Vec<u8>>();
    let result_hex = hex::encode(result);
    println!("{}", result_hex);
}