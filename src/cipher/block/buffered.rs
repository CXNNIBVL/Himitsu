use std::io;
use std::mem;
use crate::traits::cipher::{
    BlockCipherEncryption,
    BlockCipherDecryption,
};
use crate::util::{
    readable::Readable,
    buffer::FixedBuffer
};

pub struct BufferedCipherEncryption<const BLOCKSIZE: usize, T: BlockCipherEncryption<BLOCKSIZE>> {
    cipher: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>, 
    out: Vec<u8>
}

impl<const B: usize, T: BlockCipherEncryption<B>> BufferedCipherEncryption<B, T> {

    pub fn new(cipher: T) -> Self {
        Self {
            cipher,
            buffer: FixedBuffer::new(),
            out: Vec::new(),
        }
    }

    pub fn missing(&self) -> Option<usize> {

        if !self.buffer.is_full() && !self.buffer.is_empty() {
            return Some(self.buffer.capacity())
        }

        None
    }

    fn process_buffer(&mut self) {
        let mut buf = mem::replace(&mut self.buffer, FixedBuffer::new());
        self.cipher.encrypt(buf.as_mut());
        self.out.extend(buf)
    }

    pub fn finalize(self) -> Readable<Vec<u8> > {
        Readable::new(self.out)
    }

}

impl<const B: usize, T: BlockCipherEncryption<B>> io::Write for BufferedCipherEncryption<B, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {

        let mut written = 0;

        while written != buf.len() {

            written += self.buffer.push_slice(&buf[written..]);

            if self.buffer.is_full() { self.process_buffer() }
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct BufferedCipherDecryption<const BLOCKSIZE: usize, T: BlockCipherDecryption<BLOCKSIZE>> {
    cipher: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<const B: usize, T: BlockCipherDecryption<B>> BufferedCipherDecryption<B, T> {
    pub fn new(cipher: T) -> Self {
        Self {
            cipher,
            buffer: FixedBuffer::new(),
            out: Vec::new()
        }
    }

    pub fn missing(&self) -> Option<usize> {

        if !self.buffer.is_full() && !self.buffer.is_empty() {
            return Some(self.buffer.capacity())
        }

        None
    }

    fn process_buffer(&mut self) {
        let mut buf = mem::replace(&mut self.buffer, FixedBuffer::new());
        self.cipher.decrypt(buf.as_mut());
        self.out.extend(buf)
    }

    pub fn finalize(self) -> Readable<Vec<u8> > {
        Readable::new(self.out)
    }
}

impl<const B: usize, T: BlockCipherDecryption<B>> io::Write for BufferedCipherDecryption<B, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {

        let mut written = 0;

        while written != buf.len() {

            written += self.buffer.push_slice(&buf[written..]);

            if self.buffer.is_full() { self.process_buffer() }
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
