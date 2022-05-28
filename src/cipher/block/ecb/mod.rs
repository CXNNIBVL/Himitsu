mod syncronous;
mod threaded;

use crate::traits::cipher::primitive::{
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
};
pub use syncronous::{EcbEncryption, EcbDecryption};
pub use threaded::ThreadedEcb;

pub trait EcbEncryptionInjector<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveEncryption<BLOCKSIZE>;
    fn with_ecb_encryption(self) -> EcbEncryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveEncryption<B>, const B: usize> EcbEncryptionInjector<B> for T {
    type Primitive = Self;
    fn with_ecb_encryption(self) -> EcbEncryption<Self::Primitive, B> {
        EcbEncryption::new(self)
    }
}

pub trait EcbDecryptionInjector<const BLOCKSIZE: usize> {
    type Primitive: PrimitiveDecryption<BLOCKSIZE>;
    fn with_ecb_decryption(self) -> EcbDecryption<Self::Primitive, BLOCKSIZE>;
}

impl<T: PrimitiveDecryption<B>, const B: usize> EcbDecryptionInjector<B> for T {
    type Primitive = Self;
    fn with_ecb_decryption(self) -> EcbDecryption<Self::Primitive, B> {
        EcbDecryption::new(self)
    }
}

pub trait ThreadedEcbEncryptionInjector<const BLOCKSIZE: usize> {
    fn with_threaded_ecb_encryption(self, threads: usize) -> ThreadedEcb<BLOCKSIZE>;
}

impl<T, const B: usize> ThreadedEcbEncryptionInjector<B> for T
where
    T: PrimitiveEncryption<B> + Send + Sync + 'static,
{
    fn with_threaded_ecb_encryption(self, threads: usize) -> ThreadedEcb<B> {
        ThreadedEcb::encryption(self, threads)
    }
}

pub trait ThreadedEcbDecryptionInjector<const BLOCKSIZE: usize> {
    fn with_threaded_ecb_decryption(self, threads: usize) -> ThreadedEcb<BLOCKSIZE>;
}

impl<T, const B: usize> ThreadedEcbDecryptionInjector<B> for T
where
    T: PrimitiveDecryption<B> + Send + Sync + 'static,
{
    fn with_threaded_ecb_decryption(self, threads: usize) -> ThreadedEcb<B> {
        ThreadedEcb::decryption(self, threads)
    }
}
