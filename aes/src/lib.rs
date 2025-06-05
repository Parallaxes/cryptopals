use openssl::symm::{Cipher, Crypter, Mode as SymmMode};
use xor::Xor;
use rand::Rng;

static BLOCK_SIZE: usize = 16;

pub enum Mode {
    CBC,
    ECB,
    GCM,
}

#[derive(Debug)]
pub enum Aes128Error {
    EncryptionFailed,
    CrypterInitFailed,
    DecryptionFailed,
    InvalidInputPadding,
}

pub trait Aes128 {
    fn pad(&self, block_size: usize) -> Vec<u8>;
    fn unpad(&self) -> Result<Vec<u8>, Aes128Error>;
    fn padding_valid(&self) -> bool;
    fn encrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, Aes128Error>;
    fn decrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, Aes128Error>;
}

impl Aes128 for [u8] {
    fn pad(&self, block_size: usize) -> Vec<u8> {
        let padding_len = block_size - (self.len() % block_size);
        let mut padded = Vec::with_capacity(self.len() + padding_len);
        padded.extend(self);
        padded.extend(std::iter::repeat(padding_len as u8).take(padding_len));

        padded
    }

    fn unpad(&self) -> Result<Vec<u8>, Aes128Error> {
        if self.is_empty() {
            return Err(Aes128Error::InvalidInputPadding);
        }

        let pad_len = *self.last().unwrap() as usize;
        if pad_len == 0 || pad_len > self.len() {
            return Err(Aes128Error::InvalidInputPadding);
        }

        if !self[self.len() - pad_len..]
            .iter()
            .all(|&b| b as usize == pad_len)
        {
            return Err(Aes128Error::InvalidInputPadding);
        }

        Ok(self[..self.len() - pad_len].to_vec())
    }

    fn padding_valid(&self) -> bool {
        todo!()
    }

    fn encrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, Aes128Error> {
        match mode {
            Mode::CBC => self.encrypt_aes128_cbc(key, iv), // self.encrypt_aes128_cbc(key, iv),
            Mode::ECB => self.encrypt_aes128_ecb(key),
            Mode::GCM => unimplemented!(),
        }
    }

    fn decrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, Aes128Error> {
        match mode {
            Mode::CBC => self.decrypt_aes128_cbc(key, iv),
            Mode::ECB => self.decrypt_aes128_ecb(key),
            Mode::GCM => unimplemented!(),
        }
    }
}

pub trait Aes128Suite {
    fn encrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error>;
    fn decrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error>;
    fn encrypt_aes128_cbc(&self, key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, Aes128Error>;
    fn decrypt_aes128_cbc(&self, key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, Aes128Error>;
}

impl Aes128Suite for &[u8] {
    fn encrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error> {
        let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), SymmMode::Encrypt, key, None)
            .map_err(|_| Aes128Error::CrypterInitFailed)?;

        encrypter.pad(false);

        let block_size = Cipher::aes_128_ecb().block_size();
        let mut ciphertext = vec![0; self.len() + block_size];
        let mut count = encrypter
            .update(self, &mut ciphertext)
            .map_err(|_| Aes128Error::EncryptionFailed)?;
        count += encrypter
            .finalize(&mut ciphertext[count..])
            .map_err(|_| Aes128Error::EncryptionFailed)?;
        ciphertext.truncate(count);
        Ok(ciphertext)
    }

    fn decrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error> {
        let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), SymmMode::Decrypt, key, None)
            .map_err(|_| Aes128Error::CrypterInitFailed)?;

        decrypter.pad(false);

        let block_size = Cipher::aes_128_ecb().block_size();
        let mut plaintext = vec![0; self.len() + block_size];
        let mut count = decrypter
            .update(self, &mut plaintext)
            .map_err(|_| Aes128Error::DecryptionFailed)?;
        count += decrypter
            .finalize(&mut plaintext[count..])
            .map_err(|_| Aes128Error::DecryptionFailed)?;
        plaintext.truncate(count);
        Ok(plaintext)
    }

    fn encrypt_aes128_cbc(&self, key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, Aes128Error> {
        let mut result: Vec<u8> = Vec::new();
        let iv = iv.unwrap_or(&[0u8; 16]);

        let chunks = self.chunks(BLOCK_SIZE);
        let mut prev_cipher = iv.to_vec();

        for chunk in chunks {
            let block = chunk
                .fixed_xor(&prev_cipher)
                .encrypt(key, None, Mode::ECB)
                .map_err(|_| Aes128Error::EncryptionFailed)?;
            result.extend_from_slice(&block);
            prev_cipher = block;
        }

        Ok(result)
    }

    fn decrypt_aes128_cbc(&self, key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, Aes128Error> {
        let mut result: Vec<u8> = Vec::new();
        let iv = iv.unwrap_or(&[0u8; 16]);
        let mut prev_cipher = iv.to_vec();

        let chunks = self.chunks(BLOCK_SIZE);

        for chunk in chunks {
            let block = chunk
                .decrypt(key, None, Mode::ECB)
                .map_err(|_| Aes128Error::DecryptionFailed)?
                .fixed_xor(&prev_cipher);
            result.extend_from_slice(&block);
            prev_cipher = chunk.to_vec();
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serialize::{from_hex, Serialize};

    #[test]
    fn test_pkcs7_pad_exact() {
        let data = b"1234567890ABCDEF"; // 16 bytes
        let padded = data.pad(16);

        // PKCS#7: if input is a multiple of block size, add a full block of padding
        assert_eq!(padded.len(), 32);
        assert_eq!(&padded[0..16], data);
        assert_eq!(&padded[16..], &[16u8; 16]);
    }

    #[test]
    fn test_pkcs7_pad_partial() {
        let data = b"hello";
        let padded = data.pad(16);

        assert_eq!(padded.len(), 16);
        assert_eq!(&padded[0..5], b"hello");
        assert_eq!(&padded[5..], &[11u8; 11]);
    }

    #[test]
    fn test_pkcs7_unpad_invalid_len() {
        let bad = b"ac\x00";
        assert!(bad.unpad().is_err());
    }

    #[test]
    fn test_pkcs7_unpad_empty() {
        let empty = b"";
        assert!(empty.unpad().is_err());
    }

    #[test]
    fn test_encrypt_aes128_ecb_nopad() {
        let input = b"this is 16 bytes";
        let key = b"YELLOW SUBMARINE";
        let result = input.encrypt(key, None, Mode::ECB).unwrap().to_hex();
        let expected = "b8081bde98d086a0dc12220c838cf653";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_decrypt_aes128_ecb_nopad() {
        let input = from_hex("b8081bde98d086a0dc12220c838cf653").unwrap();
        let key = b"YELLOW SUBMARINE";
        let result = &input.decrypt(key, None, Mode::ECB).unwrap();
        let expected = b"this is 16 bytes";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_encrypt_aes128_cbc_nopad() {
        let input = b"this is 16 bytes";
        let key = b"YELLOW SUBMARINE";
        let iv = b"1111111111111111";
        let result = &input.encrypt(key, Some(iv), Mode::CBC).unwrap().to_hex();
        let expected = "6ba441c872d89c1e8c5dfdc0a2b3f9d1";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_decrypt_aes128_cbc_nopad() {
        let input = from_hex("6ba441c872d89c1e8c5dfdc0a2b3f9d1").unwrap();
        let key = b"YELLOW SUBMARINE";
        let iv = b"1111111111111111";
        let result = &input.decrypt(key, Some(iv), Mode::CBC).unwrap();
        let expected = b"this is 16 bytes";

        assert_eq!(result, expected);
    }
}
