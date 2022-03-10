use crate::traits::cipher::{
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption
};

use crate::cipher::blockcipher::cbc::{CbcEncryption, CbcDecryption, ThreadedCbcDecryption};

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

pub trait WithThreadedCbcDecryption<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveDecryption<BLOCKSIZE> + Send + Sync + 'static;
    fn with_threaded_cbc_decryption(self, iv: &[u8], threads: usize) -> ThreadedCbcDecryption<Self::Primitive, BLOCKSIZE>;
}

impl<T, const B: usize> WithThreadedCbcDecryption<B> for T 
    where T: PrimitiveDecryption<B> + Send + Sync + 'static
{
    type Primitive = Self;
    fn with_threaded_cbc_decryption(self, iv: &[u8], threads: usize) -> ThreadedCbcDecryption<Self::Primitive, B> {
        ThreadedCbcDecryption::new(self, iv, threads)
    }
}