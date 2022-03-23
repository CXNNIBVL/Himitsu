pub trait StreamCipherEncryption {
    fn encrypt(&mut self, data: &mut [u8]);
}

pub trait StreamCipherDecryption {
    fn decrypt(&mut self, data: &mut [u8]);
}
