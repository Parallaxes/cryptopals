mod cryptography;
use crate::cryptography::{encode};

fn main() {
    let message = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let encoded = encode::hex_to_base64(message);
    println!("{}", encoded);    
}
