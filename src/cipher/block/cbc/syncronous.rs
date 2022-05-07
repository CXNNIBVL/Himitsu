use crate::mem;
use crate::traits::cipher::{
    block::{BlockCipherDecryption, BlockCipherEncryption},
    primitive::{
        BlockCipherPrimitiveDecryption as PrimitiveDecryption,
        BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    },
};
use crate::util::secure::Array;

/// CBC Encryption Provider
pub struct CbcEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    iv: Array<u8, BLOCKSIZE>,
}

impl<T: PrimitiveEncryption<B>, const B: usize> CbcEncryption<T, B> {
    /// Create a new CBC Encryption instance from a primitive and an IV.
    /// Up to the primitives blocksize of IV contents will be used.
    pub fn new(primitive: T, iv: [u8; B]) -> Self {
        Self {
            primitive,
            iv: Array::from(iv),
        }
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> BlockCipherEncryption<B> for CbcEncryption<T, B> {
    fn encrypt(&mut self, data: &mut [u8; B]) {
        mem::xor_buffers(self.iv.as_mut(), data);
        self.primitive.encrypt(self.iv.as_mut());
        data.copy_from_slice(self.iv.as_ref());
    }
}

pub struct CbcDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    iv: Array<u8, BLOCKSIZE>,
}

impl<T: PrimitiveDecryption<B>, const B: usize> CbcDecryption<T, B> {
    pub fn new(primitive: T, iv: [u8; B]) -> Self {
        Self {
            primitive,
            iv: Array::from(iv),
        }
    }
}

impl<T: PrimitiveDecryption<B>, const B: usize> BlockCipherDecryption<B> for CbcDecryption<T, B> {
    fn decrypt(&mut self, data: &mut [u8; B]) {
        let mut new_iv = Array::<u8, B>::default();
        new_iv.copy_from_slice(data);

        self.primitive.decrypt(data);
        mem::xor_buffers(data, self.iv.as_ref());

        self.iv = new_iv;
    }
}
