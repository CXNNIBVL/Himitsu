use crate::traits::cipher::{
    block::{BlockCipherDecryption, BlockCipherEncryption},
    primitive::{
        BlockCipherPrimitiveDecryption as PrimitiveDecryption,
        BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    },
};

/// ECB encryption provider
///
/// Provides encryption in Electronic Codebook Mode based on a Primitive T eg. Aes
pub struct EcbEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
}

impl<T: PrimitiveEncryption<B>, const B: usize> EcbEncryption<T, B> {
    /// Create a new instance from a Cipher primitive
    pub fn new(primitive: T) -> Self {
        Self { primitive }
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> BlockCipherEncryption<B> for EcbEncryption<T, B> {
    fn encrypt(&mut self, data: &mut [u8; B]) {
        self.primitive.encrypt(data)
    }
}

/// ECB decryption provider
///
/// Provides decryption in Electronic Codebook Mode based on a Primitive T eg. Aes
pub struct EcbDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
}

impl<T: PrimitiveDecryption<B>, const B: usize> EcbDecryption<T, B> {
    /// Create a new instance from a Cipher primitive
    pub fn new(primitive: T) -> Self {
        Self { primitive }
    }
}

impl<T: PrimitiveDecryption<B>, const B: usize> BlockCipherDecryption<B> for EcbDecryption<T, B> {
    fn decrypt(&mut self, data: &mut [u8; B]) {
        self.primitive.decrypt(data)
    }
}
