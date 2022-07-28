pub mod aes;
pub mod serpent;
pub mod cbc;

pub trait BlockCipherEncryption<const BLOCKSIZE: usize> {
    fn encrypt(&mut self, data: &mut [u8; BLOCKSIZE]);
}

pub trait BlockCipherDecryption<const BLOCKSIZE: usize> {
    fn decrypt(&mut self, data: &mut [u8; BLOCKSIZE]);
}
