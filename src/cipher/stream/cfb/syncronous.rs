// use crate::util::buffer::FixedBuffer;
// use crate::traits::cipher::{
//     StreamCipherEncryption as StreamEncryption,
//     StreamCipherDecryption as StreamDecryption,
//     BlockCipherPrimitiveEncryption as PrimitiveEncryption
// };
// use crate::mem;

// pub struct CfbEncryption<const BLOCKSIZE: usize, T: PrimitiveEncryption<BLOCKSIZE>> {
//     primitive: T,
//     iv: FixedBuffer<u8, BLOCKSIZE>,
//     pos: usize
// }

// impl<const B: usize, T: PrimitiveEncryption<B>> CfbEncryption<B,T> {
//     pub fn new(primitive: T, iv: &[u8]) -> Self {

//         let mut iv_buf = FixedBuffer::new();
//         iv_buf.push_slice(iv);

//         Self { primitive, iv: iv_buf, pos: B }
//     }
// }

// impl<const B: usize, T: PrimitiveEncryption<B>> StreamEncryption for CfbEncryption<B,T> {
//     fn encrypt(&mut self, data: &mut [u8]) {

//         let mut encrypted = 0;

//         while encrypted < data.len() {
//             if self.pos == self.iv.len() {
//                 self.primitive.encrypt(self.iv.as_mut(), None, None);
//                 self.pos = 0;
//             }

//             let op_slice = &mut self.iv[self.pos..];
//             let xored = mem::xor_buffers(&mut data[encrypted..], op_slice);
//             op_slice.copy_from_slice(&data[encrypted..encrypted+xored]);

//             encrypted += xored;
//             self.pos += xored;
//         }
//     }
// }

// pub struct CfbDecryption<const BLOCKSIZE: usize, T: PrimitiveEncryption<BLOCKSIZE>> {
//     primitive: T,
//     iv: FixedBuffer<u8, BLOCKSIZE>,
//     pos: usize
// }

// impl<const B: usize, T: PrimitiveEncryption<B>> CfbDecryption<B,T> {
//     pub fn new(primitive: T, iv: &[u8]) -> Self {

//         let mut iv_buf = FixedBuffer::new();
//         iv_buf.push_slice(iv);

//         Self { primitive, iv: iv_buf, pos: B }
//     }
// }

// impl<const B: usize, T: PrimitiveEncryption<B>> StreamDecryption for CfbDecryption<B,T> {
//     fn decrypt(&mut self, data: &mut [u8]) {

//         let mut decrypted = 0;

//         while decrypted < data.len() {
//             if self.pos == self.iv.len() {
//                 self.primitive.encrypt(self.iv.as_mut(), None, None);
//                 self.pos = 0;
//             }

//             let min = std::cmp::min(data.len() - decrypted, self.iv.len() - self.pos);
//             let op_slice = &mut self.iv[self.pos..self.pos+min];
//             let dec_slice = &mut data[decrypted..decrypted+min];

//             dec_slice.swap_with_slice(op_slice);
//             mem::xor_buffers(dec_slice, op_slice);

//             self.pos += min;
//             decrypted += min;
//         }
//     }
// }
