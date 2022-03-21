use crate::cipher::block::buffered::{BufferedCipherDecryption, BufferedCipherEncryption};
use crate::traits::cipher::{BlockCipherDecryption, BlockCipherEncryption};

pub trait WithBufferedCipherEncryption<const BLOCKSIZE: usize> {
    type Cipher: BlockCipherEncryption<BLOCKSIZE>;
    fn buffered(self) -> BufferedCipherEncryption<BLOCKSIZE, Self::Cipher>;
}

impl<const B: usize, T: BlockCipherEncryption<B>> WithBufferedCipherEncryption<B> for T {
    type Cipher = Self;
    fn buffered(self) -> BufferedCipherEncryption<B, Self::Cipher> {
        BufferedCipherEncryption::new(self)
    }
}

pub trait WithBufferedCipherDecryption<const BLOCKSIZE: usize> {
    type Cipher: BlockCipherDecryption<BLOCKSIZE>;
    fn buffered(self) -> BufferedCipherDecryption<BLOCKSIZE, Self::Cipher>;
}

impl<const B: usize, T: BlockCipherDecryption<B>> WithBufferedCipherDecryption<B> for T {
    type Cipher = Self;
    fn buffered(self) -> BufferedCipherDecryption<B, Self::Cipher> {
        BufferedCipherDecryption::new(self)
    }
}
