use aes::Aes128;
use rand::Rng;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn encryption_oracle(input: &[u8]) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut key = [0u8; 16];

    for byte in key.iter_mut() {
        *byte = rng.random();
    }

    println!("Key: {:?}", key);

    let prefix_len = rng.random_range(5..=10);
    let suffix_len = rng.random_range(5..=10);
    let mut prefix = vec![0u8; prefix_len];
    rng.fill(&mut prefix[..]);
    let mut suffix = vec![0u8; suffix_len];
    rng.fill(&mut suffix[..]);

    let mut buf: Vec<u8> = Vec::with_capacity(input.len() + prefix_len + suffix_len);
    buf.extend_from_slice(&prefix);
    buf.extend_from_slice(&input);
    buf.extend_from_slice(&suffix);

    println!("buf: {:?}", buf);
    
    if rng.random_range(1..=2) % 2 == 0 {
        buf = buf.pad(16);
        buf.encrypt(&key, None, aes::Mode::ECB).unwrap();
    } else {
        let mut iv = vec![0u8; 16];
        rng.fill(&mut iv[..]);
        buf = buf.pad(16);
        buf.encrypt(&key, Some(&iv), aes::Mode::CBC).unwrap();
    }

    buf
}