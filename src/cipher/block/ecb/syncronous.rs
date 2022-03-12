use std::io;
use std::mem;
use crate::errors::blockcipher::BlockCipherError;
use crate::util::{
    readable::Readable,
    buffer::FixedBuffer
};
use crate::traits::cipher::{ 
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
};

/// ECB encryption provider
/// 
/// Provides encryption in Electronic Codebook Mode based on a Primitive T eg. Aes
pub struct EcbEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<T: PrimitiveEncryption<B>, const B: usize> EcbEncryption<T, B> {

    /// Create a new instance from a Cipher primitive
    pub fn new(primitive: T) -> Self {
        Self { 
            primitive,
            buffer: FixedBuffer::new(),
            out: Vec::new(),
        }
    }

    fn process_buffer(&mut self) {

        self.primitive.encrypt(self.buffer.as_mut(), None, None);

        let encrypted = mem::replace(&mut self.buffer, FixedBuffer::new());

        // Append the extracted buffer to out
        self.out.extend(encrypted);
    }

    /// Consumes the cipher, ignoring any buffered bytes and returns a Readable with the processed contents
    pub fn finalize(self) -> Readable<Vec<u8>> {
        Readable::new( self.out)
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> io::Write for EcbEncryption<T, B> {

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

/// ECB decryption provider
/// 
/// Provides decryption in Electronic Codebook Mode based on a Primitive T eg. Aes
pub struct EcbDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<T: PrimitiveDecryption<B>, const B: usize> EcbDecryption<T, B> {

    /// Create a new instance from a Cipher primitive
    pub fn new(primitive: T) -> Self {
        Self { 
            primitive,
            buffer: FixedBuffer::new(),
            out: Vec::new()
        }
    }

    fn process_buffer(&mut self) {

        self.primitive.decrypt(self.buffer.as_mut(), None, None);

        let decrypted = mem::replace(&mut self.buffer, FixedBuffer::new());

        self.out.extend(decrypted);
    }

    /// Consumes the cipher, ignoring any buffered bytes and returns a Readable with the processed contents
    pub fn finalize(self) -> Readable<Vec<u8>> {
        Readable::new( self.out)
    }
}

impl<T: PrimitiveDecryption<B>, const B: usize> io::Write for EcbDecryption<T, B> {

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