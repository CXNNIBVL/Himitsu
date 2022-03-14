use std::io;
use crate::util::readable::Readable;
pub trait BlockCipherInfo {
    const BLOCKSIZE: usize;
    fn block_size(&self) -> usize { Self::BLOCKSIZE }
}

pub trait BlockCipherEncryption<const BLOCKSIZE: usize>: BlockCipherInfo + io::Write {
    type Output: IntoIterator<Item=u8>;
    fn finalize(self) -> Readable<Self::Output>;
}

pub trait BlockCipherDecryption<const BLOCKSIZE: usize>: BlockCipherInfo + io::Write {
    type Output: IntoIterator<Item=u8>;
    fn finalize(self) -> Readable<Self::Output>;
}