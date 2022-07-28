mod syncronous;

use super::{
    BlockCipherDecryption,
    BlockCipherEncryption,
};
pub use syncronous::{CbcEncryption, CbcDecryption};

pub trait CbcEncryptionProvider<const BLOCKSIZE: usize> {
    type Primitive: BlockCipherEncryption<BLOCKSIZE>;
    fn with_cbc_encryption(self, iv: [u8; BLOCKSIZE]) -> CbcEncryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: BlockCipherEncryption<B>, const B: usize> CbcEncryptionProvider<B> for T {
    type Primitive = Self;
    fn with_cbc_encryption(self, iv: [u8; B]) -> CbcEncryption<Self::Primitive, B> {
        CbcEncryption::new(self, iv)
    }
}

pub trait CbcDecryptionProvider<const BLOCKSIZE: usize> {
    type Primitive: BlockCipherDecryption<BLOCKSIZE>;
    fn with_cbc_decryption(self, iv: [u8; BLOCKSIZE]) -> CbcDecryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: BlockCipherDecryption<B>, const B: usize> CbcDecryptionProvider<B> for T {
    type Primitive = Self;
    fn with_cbc_decryption(self, iv: [u8; B]) -> CbcDecryption<Self::Primitive, B> {
        CbcDecryption::new(self, iv)
    }
}
