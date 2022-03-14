
use crate::util::buffer::FixedBuffer;
use crate::traits::cipher::{ 
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
    BlockCipherEncryption,
    BlockCipherDecryption,
    BlockCipherInfo
};

/// CBC Encryption Provider
pub struct CbcEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    iv: FixedBuffer<u8, BLOCKSIZE>,
}

impl<T: PrimitiveEncryption<B>, const B: usize> BlockCipherInfo for CbcEncryption<T, B> {
    const BLOCKSIZE: usize = T::BLOCKSIZE;
}

impl<T: PrimitiveEncryption<B>, const B: usize> CbcEncryption<T, B> {

    /// Create a new CBC Encryption instance from a primitive and an IV.
    /// Up to the primitives blocksize of IV contents will be used.
    pub fn new(primitive: T, iv: &[u8]) -> Self {

        let mut iv_buf = FixedBuffer::new();
        iv_buf.push_slice(iv);
        
        Self { primitive, iv: iv_buf}
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> BlockCipherEncryption<B> for CbcEncryption<T, B> {
    fn encrypt(&mut self, data: &mut [u8;B]) {
        self.primitive.encrypt(data, Some(self.iv.as_ref()), None);
        self.iv.override_contents(data.as_slice(), data.len());
    }
}


pub struct CbcDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    iv: FixedBuffer<u8, BLOCKSIZE>,
}

impl<T: PrimitiveDecryption<B>, const B: usize> BlockCipherInfo for CbcDecryption<T, B> {
    const BLOCKSIZE: usize = T::BLOCKSIZE;
}

impl<T: PrimitiveDecryption<B>, const B: usize> CbcDecryption<T, B> {

    pub fn new(primitive: T, iv: &[u8]) -> Self {

        let mut iv_buf = FixedBuffer::new();
        iv_buf.push_slice(iv);
        
        Self { primitive, iv: iv_buf}
    }
}

impl<T: PrimitiveDecryption<B>, const B: usize> BlockCipherDecryption<B> for CbcDecryption<T, B> {
    fn decrypt(&mut self, data: &mut [u8;B]) {
        let mut new_iv = FixedBuffer::new();
        new_iv.push_slice(data); 
        self.primitive.decrypt(data, None, Some(self.iv.as_ref()));
        self.iv = new_iv;
    }
}