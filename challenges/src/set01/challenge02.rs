use serialize::Serialize;
use xor::Xor;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 02: ");

    let buf1 = "1c0111001f010100061a024b53535009181c";
    let buf2 = "686974207468652062756c6c277320657965";

    let result = Xor::fixed_xor(buf1, buf2).as_slice().to_hex();
    let expected = "746865206b696420646f6e277420706c6179".to_string();

    if expected == result {
        println!("Fixed XOR was successful!");
        Ok(())
    } else {
        Err("Fixed XOR failed!".into())
    }
}
