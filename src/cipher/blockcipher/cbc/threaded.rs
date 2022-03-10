
use std::io;
use std::mem;
use crate::traits::cipher::BlockCipherPrimitiveDecryption as PrimitiveDecryption;
use crate::cipher::blockcipher::primitive::threaded::ThreadedCipherDecryption as ThreadedDecryption;
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

    fn process_final(&mut self) {
        let buf = mem::replace(&mut self.buffer, FixedBuffer::new());
        let iv = mem::replace(&mut self.iv, FixedBuffer::new());

        self.primitive.put(buf.into(), None, Some(iv.into()));
    }

    /// Resets the cipher and returns a Readable with the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }
        self.process_final();

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( self.primitive.finalize() ))
    }

    /// Resets the cipher, as well as the underlying IV and returns a Readable with the processed contents
    pub fn finalize_with_iv(&mut self, iv: &[u8]) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        let out = self.finalize();

        self.iv = {
            let mut new_iv = FixedBuffer::new();
            new_iv.push_slice(iv);
            new_iv
        };

        out
    }
}

impl<T, const B: usize> io::Write for ThreadedCbcDecryption<T, B> 
    where T: PrimitiveDecryption<B> + Send + Sync + 'static
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        // Push buf until all contents have been written, if necessary, then encrypt buffer
        while written < buf.len() {

            if self.buffer.is_full() { self.process_buffer(); }

            written += self.buffer.push_slice(&buf[written..]);
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    
}