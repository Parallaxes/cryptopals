use xor::Xor;
use serialize::Serialize;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 05: ");

    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";

    let result = Xor::rep_key_xor(input, key);
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    if result.to_hex().trim() == expected {
        println!("Implementing repeating-key XOR was successful!");
        Ok(())
    } else {
        Err("Implementing repeating-key XOR failed!".into())
    }
}