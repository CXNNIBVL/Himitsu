use crate::mem;
use crate::traits::cipher::primitive::BlockCipherPrimitiveDecryption as PrimitiveDecryption;
use crate::util::{
    secure::{
        ArrayBuffer,
        Array
    },
    iopool::IoPool
};
use std::io;
use std::iter::FromIterator;

struct Transmission<const S: usize> {
    block: Array<u8, S>,
    iv: Array<u8, S>
}

type Mutator<const S: usize> = IoPool<Transmission<S>, Array<u8, S>>;

pub struct ThreadedCbcDecryption<const BLOCKSIZE: usize> {
    mutator: Mutator<BLOCKSIZE>,
    buffer: ArrayBuffer<u8, BLOCKSIZE>,
    iv: Array<u8, BLOCKSIZE>,
}

impl<const B: usize> ThreadedCbcDecryption<B> {
    pub fn new<T>(primitive: T, iv: [u8; B], threads: usize) -> Self
    where
        T: PrimitiveDecryption<B> + Send + Sync + 'static,
    {
        Self {
            mutator: Self::mutator(primitive, threads),
            buffer: ArrayBuffer::new(),
            iv: Array::from(iv)
        }
    }

    fn mutator<T>(primitive: T, threads: usize) -> Mutator<B>
    where
        T: PrimitiveDecryption<B> + Send + Sync + 'static,
    {
        Mutator::ordered_with_shared(primitive, threads, |cipher, mut msg| {
            cipher.decrypt(&mut msg.block);
            mem::xor_buffers(&mut msg.block, &msg.iv);
            msg.block
        })
    }

    fn process_buffer(&mut self) {
        let new_iv = Array::from(self.buffer).into();

        let block = self.buffer.extract();
        let iv = std::mem::replace(&mut self.iv, new_iv);

        self.mutator.push(Transmission { block, iv });
    }

    pub fn missing(&self) -> Option<usize> {
        if !self.buffer.is_full() && !self.buffer.is_empty() {
            return Some(self.buffer.capacity());
        }

        None
    }

    pub fn finalize<I>(mut self) -> I
    where
        I: FromIterator<u8>,
    {
        self.mutator.finalize().into_iter().flatten().collect()
    }

    pub fn finalize_and_reset<I>(&mut self, iv: [u8; B]) -> I
    where
        I: FromIterator<u8>
    {
        self.buffer = ArrayBuffer::new();
        self.iv = iv;

        self.mutator.finalize().into_iter().flatten().collect()
    }
}

impl<const B: usize> io::Write for ThreadedCbcDecryption<B> {
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
