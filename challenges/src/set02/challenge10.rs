use aes::Aes128;
use serialize::from_base64_file;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 02 Challenge 10: ");
    let input = from_base64_file(&"challenges/data/set02/challenge10.txt".replace("\n", ""));
    let key = b"YELLOW SUBMARINE";
    let iv = [0u8; 16];
    let result = input.decrypt(key, Some(&iv), aes::Mode::CBC).unwrap();
    let expected = b"I'm back and I'm ringin' the bell";
    if &result[0..33] == expected {
        println!("Implement CBC mode was successful!");
        Ok(())
    } else {
        Err("Implement CBC mode failed!".into())
    }
}
