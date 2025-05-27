use openssl::symm::{encrypt, Cipher};
use serialize::{from_hex, Serialize};


pub trait Encrypt {
    fn aes_128_ecb(&self, key: &[u8]) -> Vec<u8>;
}

impl Encrypt for [u8] {
    fn aes_128_ecb(&self, key: &[u8]) -> Vec<u8> {
        encrypt(Cipher::aes_128_ecb(), key, None, self).expect("AES 128 ECB encryption failed")
    }
}

pub trait Decrypt {
    fn aes_128_ecb(&self, key: &[u8]) -> Vec<u8>;
}

// pub trait Decrypt {
//     fn aes_128_ecb(key: &[u8], data: &[u8]) -> Vec<u8> {

//     }
// }



#[test]
fn test_aes_128_ecb() {
    let key = b"YELLOW SUBMARINE";
    let data = b"meow meow kitty";

    let expected = from_hex("49652E164AD1BEB085D7F3E339598CA6").unwrap();
    let result = data.aes_128_ecb(key);

    assert_eq!(expected, result);
}