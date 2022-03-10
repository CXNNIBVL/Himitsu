
use std::mem;
use std::io;
use crate::traits::cipher::{
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption
};
use crate::cipher::blockcipher::primitive::threaded::{
    ThreadedCipherEncryption as ThreadedEncryption,
    ThreadedCipherDecryption as ThreadedDecryption
};
use crate::util::{
    buffer::FixedBuffer,
    readable::Readable
};
use crate::errors::blockcipher::BlockCipherError;

pub struct ThreadedEcbEncryption<T, const BLOCKSIZE: usize> 
    where T: PrimitiveEncryption<BLOCKSIZE> + Send + Sync + 'static
{
    primitive: ThreadedEncryption<T, BLOCKSIZE>,
    buffer: FixedBuffer<u8, BLOCKSIZE>
}

impl<T, const B: usize> ThreadedEcbEncryption<T, B> 
    where T: PrimitiveEncryption<B> + Send + Sync + 'static
{
    /// Create a new instance from a Cipher primitive with the number of threads this function will use
    pub fn new(primitive: T, threads: usize) -> Self {
        Self { 
            primitive: ThreadedEncryption::new(primitive, threads),
            buffer: FixedBuffer::new()
        }
    }

    fn process_buffer(&mut self) {
        let mut_block = mem::replace(&mut self.buffer, FixedBuffer::new());
        self.primitive.put(mut_block.into(), None, None);
    }

    /// Resets the cipher and returns a Readable, containing the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock(self.buffer.capacity()) ) }

        self.process_buffer();

        let out = self.primitive.finalize();
        Ok( Readable::new(out) )
    }
}

impl<T, const B: usize> io::Write for ThreadedEcbEncryption<T,B> 
    where T: PrimitiveEncryption<B> + Send + Sync + 'static
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



pub struct ThreadedEcbDecryption<T, const BLOCKSIZE: usize> 
    where T: PrimitiveDecryption<BLOCKSIZE> + Send + Sync + 'static
{
    primitive: ThreadedDecryption<T, BLOCKSIZE>,
    buffer: FixedBuffer<u8, BLOCKSIZE>
}

impl<T, const B: usize> ThreadedEcbDecryption<T, B> 
    where T: PrimitiveDecryption<B> + Send + Sync + 'static
{

    /// Create a new instance from a Cipher primitive with the number of threads this function will use
    pub fn new(primitive: T, threads: usize) -> Self {
        Self { 
            primitive: ThreadedDecryption::new(primitive, threads),
            buffer: FixedBuffer::new()
        }
    }

    fn process_buffer(&mut self) {
        let mut_block = mem::replace(&mut self.buffer, FixedBuffer::new());
        self.primitive.put(mut_block.into(), None, None);
    }

    /// Resets the cipher and returns a Readable, containing the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock(self.buffer.capacity()) ) }
        self.process_buffer();

        let out = self.primitive.finalize();
        Ok( Readable::new(out) )
    }
}

impl<T, const B: usize> io::Write for ThreadedEcbDecryption<T,B> 
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