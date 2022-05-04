pub mod syncronous;
pub mod threaded;
use syncronous::*;
use threaded::*;
use crate::traits::cipher::primitive::BlockCipherPrimitiveEncryption as PrimitiveEncryption;

pub trait CfbEncryptionProvider<const BLOCKSIZE: usize> {
    type Cipher: PrimitiveEncryption<BLOCKSIZE>;
    fn with_cfb_encryption(self, iv: [u8; BLOCKSIZE]) -> CfbEncryption<Self::Cipher, BLOCKSIZE>;
}

impl<T: PrimitiveEncryption<B>, const B: usize> CfbEncryptionProvider<B> for T {
    type Cipher = Self;
    fn with_cfb_encryption(self, iv: [u8; B]) -> CfbEncryption<Self::Cipher, B> {
        CfbEncryption::new(self, iv)
    }
}

pub trait CfbDecryptionProvider<const BLOCKSIZE: usize> {
    type Cipher: PrimitiveEncryption<BLOCKSIZE>;
    fn with_cfb_decryption(self, iv: [u8; BLOCKSIZE]) -> CfbDecryption<Self::Cipher, BLOCKSIZE>;
}

impl<T: PrimitiveEncryption<B>, const B: usize> CfbDecryptionProvider<B> for T {
    type Cipher = Self;
    fn with_cfb_decryption(self, iv: [u8; B]) -> CfbDecryption<Self::Cipher, B> {
        CfbDecryption::new(self, iv)
    }
}
