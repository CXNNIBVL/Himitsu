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

/// CBC Encryption Provider
pub struct CbcEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    iv: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<T: PrimitiveEncryption<B>, const B: usize> CbcEncryption<T, B> {

    /// Create a new CBC Encryption instance from a primitive and an IV.
    /// Up to the primitives blocksize of IV contents will be used.
    pub fn new(primitive: T, iv: &[u8]) -> Self {

        let ( buffer, mut iv_buf ) = ( FixedBuffer::new(), FixedBuffer::new() );
        iv_buf.push_slice(iv);

        let out = Vec::new();
        
        Self { primitive, buffer, iv: iv_buf, out }
    }

    fn process_buffer(&mut self) {
        self.primitive.encrypt(self.buffer.as_mut(), Some(self.iv.as_ref()), None);
        let encrypted = mem::replace(&mut self.buffer, FixedBuffer::new());
        
        self.iv.override_contents(encrypted.as_ref(), encrypted.len());
        self.out.extend(encrypted);
    }

    fn process_final(&mut self) {
        self.primitive.encrypt(self.buffer.as_mut(), Some(self.iv.as_ref()), None);
        let encrypted = mem::replace(&mut self.buffer, FixedBuffer::new());
        
        self.out.extend(encrypted);
    }

    /// Returns a Readable with the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }
        self.process_final();

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( mem::replace(&mut self.out, Vec::new()) ))
    }

    /// Resets the cipher
    pub fn reset(&mut self, iv: &[u8]) {
        self.buffer = FixedBuffer::new();
        self.out = Vec::new();
        self.iv = {
            let mut new_iv = FixedBuffer::new();
            new_iv.push_slice(iv);
            new_iv
        };
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> io::Write for CbcEncryption<T, B> {

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


pub struct CbcDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    iv: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<T: PrimitiveDecryption<B>, const B: usize> CbcDecryption<T, B> {

    pub fn new(primitive: T, iv: &[u8]) -> Self {

        let ( buffer, mut iv_buf ) = ( FixedBuffer::new(), FixedBuffer::new() );
        iv_buf.push_slice(iv);

        let out = Vec::new();
        
        Self { primitive, buffer, iv: iv_buf, out}
    }

    fn process_buffer(&mut self) {

        let new_iv = FixedBuffer::from(self.buffer.as_ref()); 
    
        self.primitive.decrypt(self.buffer.as_mut(), None, Some(self.iv.as_ref()));
        let decrypted = mem::replace(&mut self.buffer, FixedBuffer::new());

        self.iv = new_iv;
        
        self.out.extend(decrypted);
    }

    fn process_final(&mut self) {
        
        self.primitive.decrypt(self.buffer.as_mut(), None, Some(self.iv.as_ref()));
        let decrypted = mem::replace(&mut self.buffer, FixedBuffer::new());
        
        self.out.extend(decrypted);
    }

    /// Returns a Readable with the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }
        self.process_final();

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( mem::replace(&mut self.out, Vec::new()) ))
    }

    /// Resets the cipher
    pub fn reset(&mut self, iv: &[u8]) {
        self.buffer = FixedBuffer::new();
        self.out = Vec::new();
        self.iv = {
            let mut new_iv = FixedBuffer::new();
            new_iv.push_slice(iv);
            new_iv
        };
    }
}

impl<T: PrimitiveDecryption<B>, const B: usize> io::Write for CbcDecryption<T, B> {

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
