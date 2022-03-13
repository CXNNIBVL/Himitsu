
use std::io;
use std::mem;
use crate::traits::cipher::{
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
    BlockCipherInfo,
    BlockCipherDecryption
};
use crate::cipher::block::primitive::threaded::ThreadedCipherDecryption as ThreadedDecryption;
use crate::util::{
    buffer::FixedBuffer,
    readable::Readable
};
use crate::errors::blockcipher::BlockCipherError;

pub struct ThreadedCbcDecryption<T, const BLOCKSIZE: usize> 
    where T: PrimitiveDecryption<BLOCKSIZE> + Send + Sync + 'static
{
    primitive: ThreadedDecryption<T, BLOCKSIZE>,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    iv: FixedBuffer<u8, BLOCKSIZE>,
}

impl<T, const B: usize> BlockCipherInfo for ThreadedCbcDecryption<T,B> 
    where T: PrimitiveDecryption<B> + Send + Sync + 'static
{
    const BLOCKSIZE: usize = T::BLOCKSIZE;
    const KEYLEN_MIN: usize = T::KEYLEN_MIN;
    const KEYLEN_MAX: usize = T::KEYLEN_MAX;
}

impl<T, const B: usize> ThreadedCbcDecryption<T, B> 
    where T: PrimitiveDecryption<B> + Send + Sync + 'static
{
    pub fn new(primitive: T, iv: &[u8], threads: usize) -> Self {
        let mut iv_buf = FixedBuffer::new();
        iv_buf.push_slice(iv);

        Self {
            primitive: ThreadedDecryption::new(primitive, threads),
            buffer: FixedBuffer::new(),
            iv: iv_buf
        }
    }

    fn process_buffer(&mut self) {

        let new_iv = FixedBuffer::from(self.buffer);

        let buf = mem::replace(&mut self.buffer, FixedBuffer::new());
        let iv = mem::replace(&mut self.iv, new_iv);

        self.primitive.put(buf.into(), None, Some(iv.into()));
    }
}

impl<T, const B: usize> BlockCipherDecryption<B> for ThreadedCbcDecryption<T, B> 
    where T: PrimitiveDecryption<B> + Send + Sync + 'static
{
    type Output = Vec<u8>;

    /// Returns a Readable with the processed contents
    fn finalize(mut self) -> Readable<Vec<u8>> {
        Readable::new(self.primitive.finalize())
    } 
}

impl<T, const B: usize> io::Write for ThreadedCbcDecryption<T, B> 
    where T: PrimitiveDecryption<B> + Send + Sync + 'static
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        // Push buf until all contents have been written, if necessary, then encrypt buffer
        while written < buf.len() {

            written += self.buffer.push_slice(&buf[written..]);
            
            if self.buffer.is_full() { self.process_buffer(); }
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        use io::ErrorKind;
        if !self.buffer.is_full() {
            return Err(io::Error::new(ErrorKind::UnexpectedEof, BlockCipherError::IncompleteBlock(self.buffer.capacity())))
        }

        Ok(())
    }
    
}