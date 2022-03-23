/// Adds information about common data to a blockcipher primitive
pub trait BlockCipherPrimitiveInfo {
    const BLOCKSIZE: usize;
    const KEYLEN_MIN: usize;
    const KEYLEN_MAX: usize;

    fn block_size(&self) -> usize {
        Self::BLOCKSIZE
    }
    fn keylen_min(&self) -> usize {
        Self::KEYLEN_MIN
    }
    fn keylen_max(&self) -> usize {
        Self::KEYLEN_MAX
    }
}

/// Trait for a blockcipher primitive encryption
pub trait BlockCipherPrimitiveEncryption<const BLOCKSIZE: usize>: BlockCipherPrimitiveInfo {
    fn encrypt(&self, block: &mut [u8; BLOCKSIZE]);
}

/// Trait for a blockcipher primitive decryption
pub trait BlockCipherPrimitiveDecryption<const BLOCKSIZE: usize>: BlockCipherPrimitiveInfo {
    fn decrypt(&self, block: &mut [u8; BLOCKSIZE]);
}
