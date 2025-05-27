use openssl::symm::{decrypt, encrypt, Cipher};
use serialize::{from_hex, Serialize};
use std::{error::Error, mem::uninitialized};

// pub trait Encrypt {
//     fn aes_128_ecb(&self, key: &[u8]) -> Vec<u8>;
// }

// impl Encrypt for [u8] {
//     fn aes_128_ecb(&self, key: &[u8]) -> Vec<u8> {
//         encrypt(Cipher::aes_128_ecb(), key, None, self).expect("AES 128 ECB encryption failed")
//     }
// }

// pub trait Decrypt {
//     fn aes_128_ecb(&self, key: &[u8]) -> Vec<u8>;
// }

// pub trait Decrypt {
//     fn aes_128_ecb(key: &[u8], data: &[u8]) -> Vec<u8> {

//     }
// }

// pub trait Encrypt {
//     fn encrypt(&self, key: &[u8], iv: Option<&[u8]>, mode: &str) -> Vec<u8>;
// }

// pub trait Decrypt {
//     fn decrypt(&self, key: &[u8], iv: Option<&[u8]>, mode: &str) -> Vec<u8>;
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self, key: &[u8], iv: Option<&[u8]>, mode: &str) -> Vec<u8> {
//         match mode {
//             "CBC" => {
//                 unimplemented!()
//             },

//             "ECB" => {
//                 encrypt(Cipher::aes_128_ecb(), key, None, self).expect("AES 128 ECB failed!")
//             },

//             _ => todo!()
//         }
//     }
// }

// impl Decrypt for &[u8] {
//     fn decrypt(&self, key: &[u8], iv: Option<&[u8]>, mode: &str) -> Vec<u8> {
//         match mode {
//             "CBC" => {
//                 unimplemented!()
//             },

//             "ECB" => {
//                 decrypt(Cipher::aes_128_ecb(), key, None, self).expect("AES Decryption failed!")
//             },

//             _ => todo!()
//         }
//     }
// }

static BLOCK_SIZE: usize = 16;

pub enum Mode {
    CBC,
    ECB,
    GCM,
}

pub trait Aes128 {
    fn pad(&self) -> Vec<u8>;
    fn padding_valid(&self) -> bool;
    fn encrypt(&self, key: &Self, iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, &'static str>;
    fn decrypt(&self, key: &Self, iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, &'static str>;
}

impl Aes128 for [u8] {
    fn pad(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn padding_valid(&self) -> bool {
        unimplemented!()
    }

    fn encrypt(&self, key: &Self, iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, &'static str> {
        match Mode {
            Mode::CBC => {
                unimplemented!()
            },

            Mode::ECB => {
                encrypt_aes128_ecb(&self, key)
            }
        }
    }

    fn decrypt(&self, key: &Self, iv: Option<&Self>, mode: Mode) -> Result<Vec<u8>, &'static str> {
        unimplemented!()
    }
}

fn encrypt_aes128_block(data: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.len() != BLOCK_SIZE {
        Err("Invalid block size")
    }

    Ok(vec![])
}

mod tests {
    use super::*;

    #[test]
    fn test_aes_128_ecb_encrypt() {
        let key = b"YELLOW SUBMARINE";
        let data: &[u8] = b"meow meow kitty";

        let expected = from_hex("49652E164AD1BEB085D7F3E339598CA6").unwrap();
        let result = data.encrypt(key, None, "ECB");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_aes_128_ecb_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let data_vec = from_hex("49652E164AD1BEB085D7F3E339598CA6").unwrap();
        let data: &[u8] = data_vec.as_ref();

        let expected: &[u8] = b"meow meow kitty";
        let result = data.decrypt(key, None, "ECB");

        assert_eq!(expected, result);
    }
}

