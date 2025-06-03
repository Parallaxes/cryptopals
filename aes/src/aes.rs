use openssl::symm::{Cipher, Crypter, Mode as SymmMode};
use xor::Xor;

static BLOCK_SIZE: usize = 64;

pub enum Mode {
    CBC,
    ECB,
    GCM,
}

pub enum Aes128Error {
    EncryptionFailed,
    CrypterInitFailed,
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
        ).unwrap();

        let block_size = Cipher::aes_128_ecb().block_size();
        let mut ciphertext = vec![0; self.len() + block_size];
        let mut count = encrypter.update(self, &mut ciphertext).map_err(|_| Aes128Error::EncryptionFailed)?;
        count += encrypter.finalize(&mut ciphertext[count..]).map_err(|_| Aes128Error::EncryptionFailed)?;
        ciphertext.truncate(count);
        Ok(ciphertext)
    }

    fn decrypt_aes128_ecb(&self, key: &[u8]) -> Result<Vec<u8>, Aes128Error> {
        openssl::symm::decrypt(Cipher::aes_128_ecb(), key, None, &self)
            .map_err(|_| Aes128Error::EncryptionFailed)
    }
}
