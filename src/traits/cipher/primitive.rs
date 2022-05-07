/// Trait for a blockcipher primitive encryption
pub trait BlockCipherPrimitiveEncryption<const BLOCKSIZE: usize> {
    fn encrypt(&self, block: &mut [u8; BLOCKSIZE]);
}

/// Trait for a blockcipher primitive decryption
pub trait BlockCipherPrimitiveDecryption<const BLOCKSIZE: usize> {
    fn decrypt(&self, block: &mut [u8; BLOCKSIZE]);
}
