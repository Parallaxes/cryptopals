use aes::Aes128;
use serialize::from_base64_file;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 07: ");

    let input = from_base64_file("challenges/data/set01/challenge07.txt");
    let key = b"YELLOW SUBMARINE";

    let result = input.decrypt(key, None, aes::Mode::ECB).unwrap();
    let lossy = String::from_utf8_lossy(&result);
    let parsed = lossy.lines().next().unwrap().trim();
    let expected = "I'm back and I'm ringin' the bell";

    if parsed == expected {
        println!("AES in ECB mode was successful!");
        Ok(())
    } else {
        Err("AES in ECB mode failed!".into())
    }
}
