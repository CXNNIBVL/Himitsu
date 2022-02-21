
use crate::traits::buffer::Buffer;

/// Adds information about common data (e.g Blocksize etc.) to a blockcipher primitive
pub trait BlockCipherPrimitiveInfo {
    const BLOCKSIZE: usize;
    const KEYLEN_MIN: usize;
    const KEYLEN_MAX: usize;
    type BlockType: Buffer<u8>;

    fn block_size(&self) -> usize { Self::BLOCKSIZE }
    fn keylen_min(&self) -> usize { Self::KEYLEN_MIN }
    fn keylen_max(&self) -> usize { Self::KEYLEN_MAX }
}

/// Trait for a blockcipher primitive encryption
pub trait BlockCipherPrimitiveEncryption: BlockCipherPrimitiveInfo {

    /// Creates a new blockcipher primitive
    fn new(key: &[u8]) -> Self;

    /// Mutates the mut_block, performs an xor with xor_block (if specified)
    fn mutate(&self, mut_block: &mut Self::BlockType, xor_block: Option<&Self::BlockType>);
}

/// Trait for a blockcipher primitive decryption
pub trait BlockCipherPrimitiveDecryption: BlockCipherPrimitiveInfo {

    /// Creates a new blockcipher primitive
    fn new(key: &[u8]) -> Self;

    /// Mutates the mut_block, performs an xor with xor_block (if specified)
    fn mutate(&self, mut_block: &mut Self::BlockType, xor_block: Option<&Self::BlockType>);
}