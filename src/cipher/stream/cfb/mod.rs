pub mod syncronous;
pub mod threaded;
use syncronous::*;
use threaded::*;
use crate::traits::cipher::primitive::BlockCipherPrimitiveEncryption as PrimitiveEncryption;

pub trait CfbEncryptionProvider<const BLOCKSIZE: usize> {
    type Cipher: PrimitiveEncryption<BLOCKSIZE>;
    fn with_cfb_encryption(self, iv: [u8; BLOCKSIZE]) -> CfbEncryption<BLOCKSIZE, Self::Cipher>;
}

impl<const B: usize, T: PrimitiveEncryption<B>> CfbEncryptionProvider<B> for T {
    type Cipher = Self;
    fn with_cfb_encryption(self, iv: [u8; B]) -> CfbEncryption<B, Self::Cipher> {
        CfbEncryption::new(self, iv)
    }
}

pub trait CfbDecryptionProvider<const BLOCKSIZE: usize> {
    type Cipher: PrimitiveEncryption<BLOCKSIZE>;
    fn with_cfb_decryption(self, iv: [u8; BLOCKSIZE]) -> CfbDecryption<BLOCKSIZE, Self::Cipher>;
}

impl<const B: usize, T: PrimitiveEncryption<B>> CfbDecryptionProvider<B> for T {
    type Cipher = Self;
    fn with_cfb_decryption(self, iv: [u8; B]) -> CfbDecryption<B, Self::Cipher> {
        CfbDecryption::new(self, iv)
    }
}
