use openssl::symm::Cipher;

static BLOCK_SIZE: usize = 16;

pub enum Mode {
    CBC,
    ECB,
    GCM,
}

pub trait Aes128 {
    fn pad(&self) -> Vec<u8>;
    fn padding_valid(&self) -> bool;
    fn encrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, &'static str>;
    fn decrypt(&self, key: &Self, iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, &'static str>;
}

impl Aes128 for [u8] {
    fn pad(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn padding_valid(&self) -> bool {
        unimplemented!()
    }

    fn encrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, &'static str> {
        match mode {
            Mode::CBC => {
                unimplemented!()
            }

            Mode::ECB => self.encrypt_aes128_ecb(key),

            Mode::GCM => {
                unimplemented!()
            }
        }
    }

    fn decrypt(&self, key: &Self, iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, &'static str> {
        match mode {
            Mode::CBC => {
                unimplemented!()
            }

            Mode::ECB => self.decrypt_aes128_ecb(key),

            Mode::GCM => {
                unimplemented!()
            }
        }
    }
}

pub trait Aes128Encrypt {
    fn encrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, &'static str>;
    fn decrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, &'static str>;
}

impl Aes128Encrypt for &[u8] {
    fn encrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, &'static str> {
        encrypt_aes128_block(&self, key)
    }

    fn decrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, &'static str> {
        decrypt_aes128_block(&self, key)
    }
}

fn encrypt_aes128_block(data: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    Ok(openssl::symm::encrypt(Cipher::aes_128_ecb(), key, None, data).unwrap())
}

fn decrypt_aes128_block(data: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.len() % 16 != 0 {
        return Err("Ciphertext length must be a multiple of 16 bytes");
    }

    Ok(openssl::symm::decrypt(Cipher::aes_128_ecb(), key, None, data).unwrap())
}

pub fn pkcs7_pad(input: &[u8]) -> Vec<u8> {
    assert!(BLOCK_SIZE <= 255 && BLOCK_SIZE > 0);

    let padding_len = BLOCK_SIZE - (input.len() % BLOCK_SIZE);
    let mut padded = Vec::with_capacity(input.len() + padding_len);
    padded.extend(input);
    padded.extend(std::iter::repeat(padding_len as u8).take(padding_len));

    padded
}

fn pkcs7_unpad(input: &[u8]) -> Result<Vec<u8>, &'static str> {
    if input.is_empty() {
        return Err("Input is empty");
    }

    let pad_len = *input.last().unwrap() as usize;
    if pad_len == 0 || pad_len > input.len() {
        return Err("Invalid padding length");
    }

    if !input[input.len() - pad_len..]
        .iter()
        .all(|&b| b as usize == pad_len)
    {
        return Err("Invalid PKCS#7 padding bytes");
    }

    Ok(input[..input.len() - pad_len].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serialize::from_hex;

    #[test]
    fn test_aes_128_ecb_encrypt() {
        let key = b"YELLOW SUBMARINE";
        let data: &[u8] = b"meow meow kitty";

        let expected = from_hex("49652E164AD1BEB085D7F3E339598CA6").unwrap();
        let result = data.encrypt(key, None, Mode::ECB).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_aes_128_ecb_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let data_vec = from_hex("49652E164AD1BEB085D7F3E339598CA6").unwrap();
        let data: &[u8] = data_vec.as_ref();

        let expected: &[u8] = b"meow meow kitty";
        let result = data.decrypt(key, None, Mode::ECB).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_pkcs7_pad_exact() {
        let data = b"1234567890ABCDEF"; // 16 bytes
        let padded = pkcs7_pad(data);

        assert_eq!(padded.len(), 32);
        assert_eq!(&padded[16..], &[16u8; 16]); // full block padding
    }

    #[test]
    fn test_pkcs7_pad_partial() {
        let data = b"hello";
        let padded = pkcs7_pad(data);

        assert_eq!(padded.len(), 16);
        assert_eq!(&padded[0..5], b"hello");
        assert_eq!(&padded[5..], &[11u8; 11]);
    }

    #[test]
    fn test_pkcs7_unpad_invalid_len() {
        let bad = b"ac\x00";
        assert!(pkcs7_unpad(bad).is_err());
    }

    #[test]
    fn test_pkcs7_unpad_empty() {
        let empty = b"";
        assert!(pkcs7_unpad(empty).is_err());
    }
}
