pub trait BlockCipherInfo {
    const BLOCKSIZE: usize;
    fn block_size(&self) -> usize {
        Self::BLOCKSIZE
    }
}

pub trait BlockCipherEncryption<const BLOCKSIZE: usize>: BlockCipherInfo {
    fn encrypt(&mut self, data: &mut [u8; BLOCKSIZE]);
}

pub trait BlockCipherDecryption<const BLOCKSIZE: usize>: BlockCipherInfo {
    fn decrypt(&mut self, data: &mut [u8; BLOCKSIZE]);
}
