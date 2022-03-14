use std::io;
use crate::util::{
    buffer::FixedBuffer,
    readable::Readable
};
use crate::traits::cipher::{
    StreamCipherEncryption as StreamEncryption,
    StreamCipherDecryption as StreamDecryption,
    BlockCipherPrimitiveEncryption as PrimitiveEncryption
};
use crate::mem;

pub struct CfbEncryption<const BLOCKSIZE: usize, T: PrimitiveEncryption<BLOCKSIZE>> {
    primitive: T,
    iv: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<const B: usize, T: PrimitiveEncryption<B>> io::Write for CfbEncryption<B, T> {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.out.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<const B: usize, T: PrimitiveEncryption<B>> StreamEncryption for CfbEncryption<B,T> {
    type Output = Vec<u8>;
    fn finalize(mut self) -> Readable<Self::Output> {

        let chunked = self.out.chunks_mut(B);

        for chunk in chunked {
            self.primitive.encrypt(self.iv.as_mut(), None, None);
            mem::xor_buffers(chunk, self.iv.as_ref());
            self.iv.override_contents(chunk, B);   
        }
        
        Readable::new(self.out)
    }
}

pub struct CfbDecryption<const BLOCKSIZE: usize, T: PrimitiveEncryption<BLOCKSIZE>> {
    primitive: T,
    iv: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<const B: usize, T: PrimitiveEncryption<B>> io::Write for CfbDecryption<B, T> {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.out.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<const B: usize, T: PrimitiveEncryption<B>> StreamDecryption for CfbDecryption<B,T> {
    type Output = Vec<u8>;
    fn finalize(mut self) -> Readable<Self::Output> {

        let chunked = self.out.chunks_mut(B);

        for chunk in chunked {
            
        }
        
        Readable::new(self.out)
    }
}



