use crate::cipher::block::ecb::{EcbEncryption, EcbDecryption, ThreadedEcbEncryption, ThreadedEcbDecryption};
use crate::traits::cipher::{
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption
};

pub trait WithEcbEncryption<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveEncryption<BLOCKSIZE>;
    fn with_ecb_encryption(self) -> EcbEncryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveEncryption<B>, const B: usize> WithEcbEncryption<B> for T {
    type Primitive = Self;
    fn with_ecb_encryption(self) -> EcbEncryption<Self::Primitive, B> {
        EcbEncryption::new(self)
    }
}

pub trait WithEcbDecryption<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveDecryption<BLOCKSIZE>;
    fn with_ecb_decryption(self) -> EcbDecryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveDecryption<B>, const B: usize> WithEcbDecryption<B> for T {
    type Primitive = Self;
    fn with_ecb_decryption(self) -> EcbDecryption<Self::Primitive, B> {
        EcbDecryption::new(self)
    }
}

pub trait WithThreadedEcbEncryption<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveEncryption<BLOCKSIZE> + Send + Sync + 'static;
    fn with_threaded_ecb_encryption(self, threads: usize) -> ThreadedEcbEncryption<Self::Primitive, BLOCKSIZE>;
}

impl<T, const B: usize> WithThreadedEcbEncryption<B> for T 
    where T: PrimitiveEncryption<B> + Send + Sync + 'static
{
    type Primitive = Self;
    fn with_threaded_ecb_encryption(self, threads: usize) -> ThreadedEcbEncryption<Self::Primitive, B> {
        ThreadedEcbEncryption::new(self, threads)
    }
}

pub trait WithThreadedEcbDecryption<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveDecryption<BLOCKSIZE> + Send + Sync + 'static;
    fn with_threaded_ecb_decryption(self, threads: usize) -> ThreadedEcbDecryption<Self::Primitive, BLOCKSIZE>;
}

impl<T, const B: usize> WithThreadedEcbDecryption<B> for T 
    where T: PrimitiveDecryption<B> + Send + Sync + 'static
{
    type Primitive = Self;
    fn with_threaded_ecb_decryption(self, threads: usize) -> ThreadedEcbDecryption<Self::Primitive, B> {
        ThreadedEcbDecryption::new(self, threads)
    }
}