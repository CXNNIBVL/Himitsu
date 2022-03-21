use crate::cipher::stream::cfb::{CfbEncryption, CfbDecryption};
use crate::traits::cipher::BlockCipherPrimitiveEncryption as PrimitiveEncryption;

pub trait WithCfbEncryption<const BLOCKSIZE: usize> {
    type Cipher: PrimitiveEncryption<BLOCKSIZE>;
    fn with_cfb_encryption(self, iv: &[u8]) -> CfbEncryption<BLOCKSIZE, Self::Cipher>;
}

impl<const B: usize, T: PrimitiveEncryption<B>> WithCfbEncryption<B> for T {
    type Cipher = Self;
    fn with_cfb_encryption(self, iv: &[u8]) -> CfbEncryption<B, Self::Cipher> {
        CfbEncryption::new(self, iv)
    }
}

pub trait WithCfbDecryption<const BLOCKSIZE: usize> {
    type Cipher: PrimitiveEncryption<BLOCKSIZE>;
    fn with_cfb_decryption(self, iv: &[u8]) -> CfbDecryption<BLOCKSIZE, Self::Cipher>;
}

impl<const B: usize, T: PrimitiveEncryption<B>> WithCfbDecryption<B> for T {
    type Cipher = Self;
    fn with_cfb_decryption(self, iv: &[u8]) -> CfbDecryption<B, Self::Cipher> {
        CfbDecryption::new(self, iv)
    }
}