pub use crate::traits::blockcipher::{
    BlockCipherEncryption,
    BlockCipherDecryption,
    BlockCipherInfo,
    BlockCipherResult
};

pub use crate::traits::block_primitive::{ 
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption
};

pub use std::io::{Write as IOWrite, Result as IOResult};

pub use crate::errors::blockcipher::BlockCipherError;
pub use crate::util::readable::Readable;
use crate::traits::buffer::Buffer;

pub struct EcbEncryption<T: PrimitiveEncryption> {
    primitive: T,
    buffer: T::BlockType,
    out: Vec<u8>
}

impl<T: PrimitiveEncryption> BlockCipherInfo for EcbEncryption<T> {
    const BLOCKSIZE: usize = T::BLOCKSIZE;
    const KEYLEN_MIN: usize = T::KEYLEN_MIN;
    const KEYLEN_MAX: usize = T::KEYLEN_MAX;
}

impl<T: PrimitiveEncryption> EcbEncryption<T> {
    pub fn new(key: &[u8]) -> Self {
        Self { 
            primitive: T::new(key),
            buffer: T::BlockType::new(),
            out: Vec::new(),
        }
    }
}

impl<T: PrimitiveEncryption> BlockCipherEncryption for EcbEncryption<T> {
    fn finalize(&mut self) -> BlockCipherResult {

        // If the last block is complete then encrypt
        if self.buffer.is_full() {
            self.primitive.mutate(&mut self.buffer, None, None);
        }
        // Else return error with number of missing bytes
        else if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.missing() ) ) }

        // Append the encrypted last block to out
        self.out.extend(self.buffer.extract());

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( std::mem::replace(&mut self.out, Vec::new()) ))
    }
}

impl<T: PrimitiveEncryption> IOWrite for EcbEncryption<T> {

    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        let mut written = 0;
        for element in buf {

            // If push fails, then the buffer is full
            if !self.buffer.push(element) {
                // Proceed to encrypt the buffer
                self.primitive.mutate(&mut self.buffer, None, None);

                // Push buffer contents into out
                self.out.extend(self.buffer.extract());

                // Retry pushing the element
                let _ = self.buffer.push(element);
            }

            written += 1;
        }

        Ok(written)
    }

    fn flush(&mut self) -> IOResult<()> {
        Ok(())
    }
    
}

pub struct EcbDecryption<T: PrimitiveDecryption> {
    primitive: T,
    buffer: T::BlockType,
    out: Vec<u8>
}

impl<T: PrimitiveDecryption> BlockCipherInfo for EcbDecryption<T> {
    const BLOCKSIZE: usize = T::BLOCKSIZE;
    const KEYLEN_MIN: usize = T::KEYLEN_MIN;
    const KEYLEN_MAX: usize = T::KEYLEN_MAX;
}

impl<T: PrimitiveDecryption> EcbDecryption<T> {
    pub fn new(key: &[u8]) -> Self {
        Self { 
            primitive: T::new(key),
            buffer: T::BlockType::new(),
            out: Vec::new()
        }
    }
}

impl<T: PrimitiveDecryption> BlockCipherDecryption for EcbDecryption<T> {
    fn finalize(&mut self) -> BlockCipherResult {
        // If the last block is complete then encrypt
        if self.buffer.is_full() {
            self.primitive.mutate(&mut self.buffer, None, None);
        }
        // Else return error with number of missing bytes
        else if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.missing() ) ) }

        // Append the encrypted last block to out
        self.out.extend(self.buffer.extract());

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( std::mem::replace(&mut self.out, Vec::new()) ))
    }
}

impl<T: PrimitiveDecryption> IOWrite for EcbDecryption<T> {

    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        let mut written = 0;
        for element in buf {

            // If push fails, then the buffer is full
            if !self.buffer.push(element) {
                // Proceed to encrypt the buffer
                self.primitive.mutate(&mut self.buffer, None, None);

                // Push buffer contents into out
                self.out.extend(self.buffer.extract());

                // Retry pushing the element
                let _ = self.buffer.push(element);
            }

            written += 1;
        }

        Ok(written)
    }

    fn flush(&mut self) -> IOResult<()> {
        Ok(())
    }
    
}


#[cfg(test)]
mod tests {

    use crate::cipher::blockcipher::primitive::aes;
    use super::*;

    #[test]
    fn api_test() {

        let key = Vec::new();
        let _ciph_api1 = EcbEncryption::<aes::Aes>::new(&key);
        let _info = EcbEncryption::<aes::Aes>::KEYLEN_MIN;
    }

}