pub mod syncronous;
pub mod threaded;

use syncronous::*;
use threaded::*;
use crate::traits::cipher::primitive::{
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
};

pub trait CbcEncryptionProvider<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveEncryption<BLOCKSIZE>;
    fn with_cbc_encryption(self, iv: [u8; BLOCKSIZE]) -> CbcEncryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveEncryption<B>, const B: usize> CbcEncryptionProvider<B> for T {
    type Primitive = Self;
    fn with_cbc_encryption(self, iv: [u8; B]) -> CbcEncryption<Self::Primitive, B> {
        CbcEncryption::new(self, iv)
    }
}

pub trait CbcDecryptionProvider<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveDecryption<BLOCKSIZE>;
    fn with_cbc_decryption(self, iv: [u8; BLOCKSIZE]) -> CbcDecryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveDecryption<B>, const B: usize> CbcDecryptionProvider<B> for T {
    type Primitive = Self;
    fn with_cbc_decryption(self, iv: [u8; B]) -> CbcDecryption<Self::Primitive, B> {
        CbcDecryption::new(self, iv)
    }
}

pub trait ThreadedCbcDecryptionProvider<const BLOCKSIZE: usize> {
    fn with_threaded_cbc_decryption(
        self,
        iv: [u8; BLOCKSIZE],
        threads: usize,
    ) -> ThreadedCbcDecryption<BLOCKSIZE>;
}

impl<T, const B: usize> ThreadedCbcDecryptionProvider<B> for T
where
    T: PrimitiveDecryption<B> + Send + Sync + 'static,
{
    fn with_threaded_cbc_decryption(self, iv: [u8; B], threads: usize) -> ThreadedCbcDecryption<B> {
        ThreadedCbcDecryption::new::<T>(self, iv, threads)
    }
}