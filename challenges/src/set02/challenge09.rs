use aes::Aes128;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 02 Challenge 09: ");

    let input = b"YELLOW SUBMARINE";
    let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";

    let result = input.pad(20);

    if result == expected {
        println!("Implement PKCS#7 Padding was successful!");
        Ok(())
    } else {
        Err("Implement PKCS#7 Padding failfed!".into())
    }
}
