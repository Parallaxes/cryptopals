use openssl::symm::{Cipher, Crypter, Mode as SymmMode};
use xor::Xor;

static BLOCK_SIZE: usize = 64;

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
}

pub trait Aes128 {
    fn pad(&self) -> Vec<u8>;
    fn padding_valid(&self) -> bool;
    fn encrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, Aes128Error>;
    fn decrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, Aes128Error>;
}

impl Aes128 for [u8] {
    fn pad(&self) -> Vec<u8> {
        todo!()
    }

    fn padding_valid(&self) -> bool {
        todo!()
    }

    fn encrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, Aes128Error> {
        match mode {
            Mode::CBC => unimplemented!(), // self.encrypt_aes128_cbc(key, iv),
            Mode::ECB => self.encrypt_aes128_ecb(key),
            Mode::GCM => unimplemented!(),
        }
    }

    fn decrypt(&self, key: &[u8], iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, Aes128Error> {
        match mode {
            Mode::CBC => unimplemented!(),
            Mode::ECB => self.decrypt_aes128_ecb(key),
            Mode::GCM => unimplemented!(),
        }
    }
}

pub trait Aes128Suite {
    fn encrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error>;
    fn decrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error>;
}

impl Aes128Suite for &[u8] {
    fn encrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error> {
        let mut encrypter = Crypter::new(
            Cipher::aes_128_ecb(),
            SymmMode::Encrypt,
            key,
            None
        ).map_err(|_| Aes128Error::CrypterInitFailed)?;

        encrypter.pad(false);

        let block_size = Cipher::aes_128_ecb().block_size();
        let mut ciphertext = vec![0; self.len() + block_size];
        let mut count = encrypter.update(self, &mut ciphertext).map_err(|_| Aes128Error::EncryptionFailed)?;
        count += encrypter.finalize(&mut ciphertext[count..]).map_err(|_| Aes128Error::EncryptionFailed)?;
        ciphertext.truncate(count);
        Ok(ciphertext)
    }

    fn decrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error> {
        let mut decrypter = Crypter::new(
            Cipher::aes_128_ecb(),
            SymmMode::Decrypt,
            key,
            None
        ).map_err(|_| Aes128Error::CrypterInitFailed)?;

        decrypter.pad(false);

        let block_size = Cipher::aes_128_ecb().block_size();
        let mut plaintext = vec![0; self.len() + block_size];
        let mut count = decrypter.update(self, &mut plaintext).map_err(|_| Aes128Error::DecryptionFailed)?;
        count += decrypter.finalize(&mut plaintext[count..]).map_err(|_| Aes128Error::DecryptionFailed)?;
        plaintext.truncate(count);
        Ok(plaintext)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serialize::{Serialize, from_hex};

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
}