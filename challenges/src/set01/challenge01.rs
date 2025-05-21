use serialize::{Serialize, from_hex, string_from_vec};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 01: ");

    let input = Serialize::to_base64(string_from_vec(from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap()).as_str()).unwrap();
    let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    
    if expected == input {
        println!("Convert hex to base64 successful!");
        Ok(())
    } else {
        Err("Convert hex to base64 failed!".into())
    }
}