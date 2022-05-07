use crate::mem;
use crate::traits::cipher::{
    primitive::BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    stream::{
        StreamCipherDecryption as StreamDecryption, StreamCipherEncryption as StreamEncryption,
    },
};
use crate::util::secure::Array;

pub struct CfbEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    iv: Array<u8, BLOCKSIZE>,
    pos: usize,
}

impl<T: PrimitiveEncryption<B>, const B: usize> CfbEncryption<T, B> {
    pub(super) fn new(primitive: T, iv: [u8; B]) -> Self {
        Self {
            primitive,
            iv: Array::from(iv),
            pos: B,
        }
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> StreamEncryption for CfbEncryption<T, B> {
    fn encrypt(&mut self, data: &mut [u8]) {
        let mut encrypted = 0;

        while encrypted < data.len() {
            if self.pos == B {
                self.primitive.encrypt(&mut self.iv);
                self.pos = 0;
            }

            let op_slice = &mut self.iv[self.pos..];
            let xored = mem::xor_buffers(&mut data[encrypted..], op_slice);
            op_slice.copy_from_slice(&data[encrypted..encrypted + xored]);

            encrypted += xored;
            self.pos += xored;
        }
    }
}

pub struct CfbDecryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    iv: Array<u8, BLOCKSIZE>,
    pos: usize,
}

impl<T: PrimitiveEncryption<B>, const B: usize> CfbDecryption<T, B> {
    pub(super) fn new(primitive: T, iv: [u8; B]) -> Self {
        Self {
            primitive,
            iv: Array::from(iv),
            pos: B,
        }
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> StreamDecryption for CfbDecryption<T, B> {
    fn decrypt(&mut self, data: &mut [u8]) {
        let mut decrypted = 0;

        while decrypted < data.len() {
            if self.pos == B {
                self.primitive.encrypt(&mut self.iv);
                self.pos = 0;
            }

            let min = std::cmp::min(data.len() - decrypted, self.iv.len() - self.pos);
            let op_slice = &mut self.iv[self.pos..self.pos + min];
            let dec_slice = &mut data[decrypted..decrypted + min];

            dec_slice.swap_with_slice(op_slice);
            mem::xor_buffers(dec_slice, op_slice);

            self.pos += min;
            decrypted += min;
        }
    }
}
