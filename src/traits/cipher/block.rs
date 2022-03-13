use std::io;
use crate::util::readable::Readable;
pub trait BlockCipherInfo {
    const BLOCKSIZE: usize;
    const KEYLEN_MIN: usize;
    const KEYLEN_MAX: usize;

    fn block_size(&self) -> usize { Self::BLOCKSIZE }
    fn keylen_min(&self) -> usize { Self::KEYLEN_MIN }
    fn keylen_max(&self) -> usize { Self::KEYLEN_MAX }
}

pub trait BlockCipherEncryption<const BLOCKSIZE: usize>: BlockCipherInfo + io::Write {
    type Output: IntoIterator<Item=u8>;
    fn finalize(self) -> Readable<Self::Output>;
}

pub trait BlockCipherDecryption<const BLOCKSIZE: usize>: BlockCipherInfo + io::Write {
    type Output: IntoIterator<Item=u8>;
    fn finalize(self) -> Readable<Self::Output>;
}