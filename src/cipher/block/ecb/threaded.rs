use crate::traits::cipher::primitive::{
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
};
use crate::util::{buffer::ArrayBuffer, iopool::IoPool};
use std::io;
use std::iter::FromIterator;

type Mutator<const BLOCKSIZE: usize> = IoPool<[u8; BLOCKSIZE], [u8; BLOCKSIZE]>;

pub struct ThreadedEcb<const BLOCKSIZE: usize> {
    mutator: Mutator<BLOCKSIZE>,
    buffer: ArrayBuffer<u8, BLOCKSIZE>,
}

impl<const B: usize> ThreadedEcb<B> {
    pub fn encryption<T>(primitive: T, threads: usize) -> Self
    where
        T: PrimitiveEncryption<B> + Send + Sync + 'static,
    {
        let mutator = Self::encryptor(primitive, threads);

        Self {
            mutator,
            buffer: ArrayBuffer::new(),
        }
    }

    pub fn decryption<T>(primitive: T, threads: usize) -> Self
    where
        T: PrimitiveDecryption<B> + Send + Sync + 'static,
    {
        let mutator = Self::decryptor(primitive, threads);

        Self {
            mutator,
            buffer: ArrayBuffer::new(),
        }
    }

    fn encryptor<T>(primitive: T, threads: usize) -> Mutator<B>
    where
        T: PrimitiveEncryption<B> + Send + Sync + 'static,
    {
        Mutator::ordered_with_shared(primitive, threads, |cipher, mut block| {
            cipher.encrypt(&mut block);
            block
        })
    }

    fn decryptor<T>(primitive: T, threads: usize) -> Mutator<B>
    where
        T: PrimitiveDecryption<B> + Send + Sync + 'static,
    {
        Mutator::ordered_with_shared(primitive, threads, |cipher, mut block| {
            cipher.decrypt(&mut block);
            block
        })
    }

    fn process_buffer(&mut self) {
        let block = self.buffer.extract();
        self.mutator.push(block);
    }

    pub fn missing(&self) -> Option<usize> {
        if !self.buffer.is_full() && !self.buffer.is_empty() {
            return Some(self.buffer.capacity());
        }

        None
    }

    /// Consumes the cipher, ignoring any buffered bytes and returns a Readable with the processed contents
    pub fn finalize<I>(mut self) -> I
    where
        I: FromIterator<u8>,
    {
        self.mutator.finalize().into_iter().flatten().collect()
    }

    pub fn finalize_and_reset<I>(&mut self) -> I
    where I: FromIterator<u8>
    {
        self.buffer = ArrayBuffer::new();
        self.mutator.finalize().into_iter().flatten().collect()
    }
}

impl<const B: usize> io::Write for ThreadedEcb<B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        // Push buf until all contents have been written, if necessary, then encrypt buffer
        while written < buf.len() {
            written += self.buffer.push_slice(&buf[written..]);

            if self.buffer.is_full() {
                self.process_buffer();
            }
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
