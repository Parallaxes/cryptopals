use std::collections::HashSet;
use std::fs;

use serialize::from_hex;
use serialize::Serialize;

static BLOCK_SIZE: usize = 16;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!("Set 01 Challenge 08: ");

    let input =
        fs::read_to_string("challenges/data/set01/challenge08.txt").expect("Failed to read file");

    let expected = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5\
             d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649\
             af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c\
             7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6\
             aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd2\
             83d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c\
             58386b06fba186a";
    let result = detect_ecb(input).to_hex();

    if result == expected {
        println!("Detect ECB in AES mode was successful!");
        Ok(())
    } else {
        Err("Detect AES in ECB mode failed!".into())
    }
}

// ECB is determinstic, so the same 16 byte plaintext block will produce the same 16 byte ciphertext block.
// Thus, for each block we deserialize the hex data and place each 17 byte block into a HashSet. If the insertion 
// action fails, we know that there is a duplicate and that block is the duplicate (ECB).
fn detect_ecb(input: String) -> Vec<u8> {
    for line in input.lines() {
        let mut seen: HashSet<Vec<u8>> = HashSet::new();
        let deserialized = from_hex(line).unwrap();

        for chunk in deserialized.chunks(BLOCK_SIZE) {
            if !seen.insert(chunk.to_vec()) {
                return deserialized;
            }
        }
    }

    vec![]
}
