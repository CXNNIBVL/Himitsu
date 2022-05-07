use crate::traits::cipher::block::{BlockCipherDecryption, BlockCipherEncryption};
use crate::traits::util::buffer::Buffer;
use crate::util::secure::{ArrayBuffer, Vector};
use std::io;
use std::iter::FromIterator;

pub trait BufferedCipherEncryptionProvider<const BLOCKSIZE: usize> {
    type Cipher: BlockCipherEncryption<BLOCKSIZE>;
    fn buffered(self) -> BufferedCipherEncryption<Self::Cipher, BLOCKSIZE>;
}

impl<T: BlockCipherEncryption<B>, const B: usize> BufferedCipherEncryptionProvider<B> for T {
    type Cipher = Self;
    fn buffered(self) -> BufferedCipherEncryption<Self::Cipher, B> {
        BufferedCipherEncryption::new(self)
    }
}

pub trait BufferedCipherDecryptionProvider<const BLOCKSIZE: usize> {
    type Cipher: BlockCipherDecryption<BLOCKSIZE>;
    fn buffered(self) -> BufferedCipherDecryption<Self::Cipher, BLOCKSIZE>;
}

impl<T: BlockCipherDecryption<B>, const B: usize> BufferedCipherDecryptionProvider<B> for T {
    type Cipher = Self;
    fn buffered(self) -> BufferedCipherDecryption<Self::Cipher, B> {
        BufferedCipherDecryption::new(self)
    }
}

pub struct BufferedCipherEncryption<T: BlockCipherEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    cipher: T,
    buffer: ArrayBuffer<u8, BLOCKSIZE>,
    out: Vector<u8>,
}

impl<T: BlockCipherEncryption<B>, const B: usize> BufferedCipherEncryption<T, B> {
    pub fn new(cipher: T) -> Self {
        Self {
            cipher,
            buffer: ArrayBuffer::new(),
            out: Vector::new(),
        }
    }

    pub fn missing(&self) -> Option<usize> {
        if !self.buffer.is_full() && !self.buffer.is_empty() {
            return Some(self.buffer.capacity());
        }

        None
    }

    fn process_buffer(&mut self) {
        let mut buf = self.buffer.extract();
        self.cipher.encrypt(&mut buf);
        self.out.extend(buf.into_iter())
    }

    pub fn finalize<I>(self) -> I
    where
        I: FromIterator<u8>,
    {
        self.out.into_iter().collect()
    }
}

impl<T: BlockCipherEncryption<B>, const B: usize> io::Write for BufferedCipherEncryption<T, B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        while written != buf.len() {
            written += self.buffer.push_slice(&buf[written..]);

            if self.buffer.is_full() {
                self.process_buffer()
            }
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct BufferedCipherDecryption<T: BlockCipherDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    cipher: T,
    buffer: ArrayBuffer<u8, BLOCKSIZE>,
    out: Vector<u8>,
}

impl<T: BlockCipherDecryption<B>, const B: usize> BufferedCipherDecryption<T, B> {
    pub fn new(cipher: T) -> Self {
        Self {
            cipher,
            buffer: ArrayBuffer::new(),
            out: Vector::new(),
        }
    }

    pub fn missing(&self) -> Option<usize> {
        if !self.buffer.is_full() && !self.buffer.is_empty() {
            return Some(self.buffer.capacity());
        }

        None
    }

    fn process_buffer(&mut self) {
        let mut buf = self.buffer.extract();
        self.cipher.decrypt(&mut buf);
        self.out.extend(buf.into_iter())
    }

    pub fn finalize<I>(self) -> I
    where
        I: FromIterator<u8>,
    {
        self.out.into_iter().collect()
    }
}

impl<T: BlockCipherDecryption<B>, const B: usize> io::Write for BufferedCipherDecryption<T, B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        while written != buf.len() {
            written += self.buffer.push_slice(&buf[written..]);

            if self.buffer.is_full() {
                self.process_buffer()
            }
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
