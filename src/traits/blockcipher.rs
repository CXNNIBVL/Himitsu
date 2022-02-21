use std::io;
use crate::util::readable::Readable;
use crate::errors::blockcipher::BlockCipherError;

pub type BlockCipherResult = Result<Readable<Vec<u8>>, BlockCipherError>;

/// Adds information about common data (e.g Blocksize etc.) to a blockcipher
pub trait BlockCipherInfo {
    const BLOCKSIZE: usize;
    const KEYLEN_MIN: usize;
    const KEYLEN_MAX: usize;

    fn block_size(&self) -> usize { Self::BLOCKSIZE }
    fn keylen_min(&self) -> usize { Self::KEYLEN_MIN }
    fn keylen_max(&self) -> usize { Self::KEYLEN_MAX }
}

/// Trait for blockcipher encryption
pub trait BlockCipherEncryption: BlockCipherInfo + io::Write {
    /// Resets the blockcipher and returns a Readable or an Error
    fn finalize(&mut self) -> BlockCipherResult;
}

/// Trait for blockcipher decryption
pub trait BlockCipherDecryption: BlockCipherInfo + io::Write {
    /// Resets the blockcipher and returns a Readable or an Error
    fn finalize(&mut self) -> BlockCipherResult;
}