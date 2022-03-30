use crate::cipher::block::ecb::{EcbDecryption, EcbEncryption, ThreadedEcb};
use crate::traits::cipher::primitive::{
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
};

pub trait EcbEncryptionProvider<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveEncryption<BLOCKSIZE>;
    fn with_ecb_encryption(self) -> EcbEncryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveEncryption<B>, const B: usize> EcbEncryptionProvider<B> for T {
    type Primitive = Self;
    fn with_ecb_encryption(self) -> EcbEncryption<Self::Primitive, B> {
        EcbEncryption::new(self)
    }
}

pub trait EcbDecryptionProvider<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveDecryption<BLOCKSIZE>;
    fn with_ecb_decryption(self) -> EcbDecryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveDecryption<B>, const B: usize> EcbDecryptionProvider<B> for T {
    type Primitive = Self;
    fn with_ecb_decryption(self) -> EcbDecryption<Self::Primitive, B> {
        EcbDecryption::new(self)
    }
}

pub trait ThreadedEcbEncryptionProvider<const BLOCKSIZE: usize> {
    fn with_threaded_ecb_encryption(self, threads: usize) -> ThreadedEcb<BLOCKSIZE>;
}

impl<T, const B: usize> ThreadedEcbEncryptionProvider<B> for T
where
    T: PrimitiveEncryption<B> + Send + Sync + 'static,
{
    fn with_threaded_ecb_encryption(self, threads: usize) -> ThreadedEcb<B> {
        ThreadedEcb::encryption(self, threads)
    }
}

pub trait ThreadedEcbDecryptionProvider<const BLOCKSIZE: usize> {
    fn with_threaded_ecb_decryption(self, threads: usize) -> ThreadedEcb<BLOCKSIZE>;
}

impl<T, const B: usize> ThreadedEcbDecryptionProvider<B> for T
where
    T: PrimitiveDecryption<B> + Send + Sync + 'static,
{
    fn with_threaded_ecb_decryption(self, threads: usize) -> ThreadedEcb<B> {
        ThreadedEcb::decryption(self, threads)
    }
}
