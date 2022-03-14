use crate::traits::cipher::{ 
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
    BlockCipherInfo,
    BlockCipherEncryption,
    BlockCipherDecryption
};

/// ECB encryption provider
/// 
/// Provides encryption in Electronic Codebook Mode based on a Primitive T eg. Aes
pub struct EcbEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
}

impl<T: PrimitiveEncryption<B>, const B: usize> BlockCipherInfo for EcbEncryption<T, B> {
    const BLOCKSIZE: usize = T::BLOCKSIZE;
}

impl<T: PrimitiveEncryption<B>, const B: usize> EcbEncryption<T, B> {

    /// Create a new instance from a Cipher primitive
    pub fn new(primitive: T) -> Self {
        Self { primitive }
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> BlockCipherEncryption<B> for EcbEncryption<T, B> {
    fn encrypt(&mut self, data: &mut [u8;B]) {
        self.primitive.encrypt(data, None, None)
    }
}

/// ECB decryption provider
/// 
/// Provides decryption in Electronic Codebook Mode based on a Primitive T eg. Aes
pub struct EcbDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
}

impl<T: PrimitiveDecryption<B>, const B: usize> BlockCipherInfo for EcbDecryption<T, B> {
    const BLOCKSIZE: usize = T::BLOCKSIZE;
}

impl<T: PrimitiveDecryption<B>, const B: usize> EcbDecryption<T, B> {

    /// Create a new instance from a Cipher primitive
    pub fn new(primitive: T) -> Self {
        Self { primitive }
    }
}

impl<T: PrimitiveDecryption<B>, const B: usize> BlockCipherDecryption<B> for EcbDecryption<T, B> {
    fn decrypt(&mut self, data: &mut [u8;B]) {
        self.primitive.decrypt(data, None, None)
    }
}