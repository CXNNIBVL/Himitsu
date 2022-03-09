use crate::traits::cipher::{
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption
};

use crate::cipher::blockcipher::cbc::{CbcEncryption, CbcDecryption};

pub trait WithCbcEncryption<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveEncryption<BLOCKSIZE>;
    fn with_cbc_encryption(self, iv: &[u8]) -> CbcEncryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveEncryption<B>, const B: usize> WithCbcEncryption<B> for T {
    type Primitive = Self;
    fn with_cbc_encryption(self, iv: &[u8]) -> CbcEncryption<Self::Primitive, B> {
        CbcEncryption::new(self, iv)
    }
}

pub trait WithCbcDecryption<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveDecryption<BLOCKSIZE>;
    fn with_cbc_decryption(self, iv: &[u8]) -> CbcDecryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveDecryption<B>, const B: usize> WithCbcDecryption<B> for T {
    type Primitive = Self;
    fn with_cbc_decryption(self, iv: &[u8]) -> CbcDecryption<Self::Primitive, B> {
        CbcDecryption::new(self, iv)
    }
}