use crate::traits::blockcipher::{
    BlockCipherEncryption,
    BlockCipherDecryption,
    BlockCipherInfo,
    BlockCipherResult
};
use std::io::{Write as ioWrite, Result as ioResult};
use std::mem;
use crate::errors::blockcipher::BlockCipherError;
use crate::util::readable::Readable;
use crate::traits::blockcipher_primitive::{ 
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
};
use crate::traits::buffer::Buffer;

pub struct CbcEncryption<T: PrimitiveEncryption> {
    primitive: T,
    buffer: T::BlockType,
    iv: T::BlockType,
    out: Vec<u8>
}

impl<T: PrimitiveEncryption> BlockCipherInfo for CbcEncryption<T> {
    const BLOCKSIZE: usize = T::BLOCKSIZE;
    const KEYLEN_MIN: usize = T::KEYLEN_MIN;
    const KEYLEN_MAX: usize = T::KEYLEN_MAX;
}

impl<T: PrimitiveEncryption> CbcEncryption<T> {

    pub fn new(key: &[u8], iv: &[u8]) -> Self {

        let primitive = T::new(key);
        let ( buffer, mut iv_buf ) = ( T::new_block(), T::new_block() );
        iv_buf.push_slice(iv);

        let out = Vec::new();
        
        Self { primitive, buffer, iv: iv_buf, out}
    }

    fn process_buffer(&mut self) {
        self.primitive.mutate(&mut self.buffer, Some(&self.iv), None);
        let encrypted = mem::replace(&mut self.buffer, T::new_block());
        
        self.iv.override_contents(encrypted.as_slice(), encrypted.len());
        self.out.extend(encrypted);
    }

    fn process_final(&mut self) {
        self.primitive.mutate(&mut self.buffer, Some(&self.iv), None);
        let encrypted = mem::replace(&mut self.buffer, T::new_block());
        
        self.out.extend(encrypted);
    }
}

impl<T: PrimitiveEncryption> BlockCipherEncryption for CbcEncryption<T> {
    fn finalize(&mut self) -> BlockCipherResult {

        // If the last block is complete then encrypt
        if self.buffer.is_full() { self.process_final(); }
        // Else return error with number of missing bytes
        else if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( mem::replace(&mut self.out, Vec::new()) ))
    }
}

impl<T: PrimitiveEncryption> ioWrite for CbcEncryption<T> {

    fn write(&mut self, buf: &[u8]) -> ioResult<usize> {
        let mut written = 0;

        // Push buf until all contents have been written, if necessary, then encrypt buffer
        while written < buf.len() {

            if self.buffer.is_full() { self.process_buffer(); }

            written += self.buffer.push_slice(&buf[written..]);
        }

        Ok(written)
    }

    fn flush(&mut self) -> ioResult<()> {
        Ok(())
    }
    
}


pub struct CbcDecryption<T: PrimitiveDecryption> {
    primitive: T,
    buffer: T::BlockType,
    iv: T::BlockType,
    out: Vec<u8>
}

impl<T: PrimitiveDecryption> BlockCipherInfo for CbcDecryption<T> {
    const BLOCKSIZE: usize = T::BLOCKSIZE;
    const KEYLEN_MIN: usize = T::KEYLEN_MIN;
    const KEYLEN_MAX: usize = T::KEYLEN_MAX;
}

impl<T: PrimitiveDecryption> CbcDecryption<T> {

    pub fn new(key: &[u8], iv: &[u8]) -> Self {

        let primitive = T::new(key);
        let ( buffer, mut iv_buf ) = ( T::new_block(), T::new_block() );
        iv_buf.push_slice(iv);

        let out = Vec::new();
        
        Self { primitive, buffer, iv: iv_buf, out}
    }

    fn process_buffer(&mut self) {

        let new_iv = {
            let mut iv = T::new_block();
            iv.push_slice(self.buffer.as_slice());
            iv
        };
        
        self.primitive.mutate(&mut self.buffer, None, Some(&self.iv));
        let decrypted = mem::replace(&mut self.buffer, T::new_block());

        self.iv = new_iv;
        
        self.out.extend(decrypted);
    }

    fn process_final(&mut self) {
        
        self.primitive.mutate(&mut self.buffer, None, Some(&self.iv));
        let decrypted = mem::replace(&mut self.buffer, T::new_block());
        
        self.out.extend(decrypted);
    }
}

impl<T: PrimitiveDecryption> BlockCipherDecryption for CbcDecryption<T> {
    fn finalize(&mut self) -> BlockCipherResult {

        // If the last block is complete then encrypt
        if self.buffer.is_full() { self.process_final(); }
        // Else return error with number of missing bytes
        else if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( mem::replace(&mut self.out, Vec::new()) ))
    }
}

impl<T: PrimitiveDecryption> ioWrite for CbcDecryption<T> {

    fn write(&mut self, buf: &[u8]) -> ioResult<usize> {
        let mut written = 0;

        // Push buf until all contents have been written, if necessary, then encrypt buffer
        while written < buf.len() {

            if self.buffer.is_full() { self.process_buffer(); }

            written += self.buffer.push_slice(&buf[written..]);
        }

        Ok(written)
    }

    fn flush(&mut self) -> ioResult<()> {
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {

    use std::io::Read;
    use crate::cipher::blockcipher::primitive::aes;
    use super::*;

    fn decode(s: &str) -> Vec<u8> {
		use crate::encode::HexEncoder;
		HexEncoder::builder().decode(s)
	}

    macro_rules! cbc_test {
        (
            $fn_name: ident,
            $cipher: ty,
            $key: literal,
            $iv: literal,
            $input: literal,
            $expected: literal
        ) => {

            #[test]
            fn $fn_name() {

                let input = decode($input);
                let key = decode($key);
                let iv = decode($iv);
                let expected = decode($expected);

                let mut cipher = <$cipher>::new(&key, &iv);
                cipher.write_all(&input).unwrap();

                let mut reader = cipher.finalize().unwrap();

                let mut output = Vec::new();
                reader.read_to_end(&mut output).unwrap();

                assert_eq!(expected, output);

            }
            
        };
    }

    // Example values from [NIST](https://csrc.nist.gov/projects/cryptographic-standards-and-guidelines/example-values)

    cbc_test!(
        test_cbc_aes128_enc,
        CbcEncryption<aes::Aes>,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "7649ABAC 8119B246 CEE98E9B 12E9197D 5086CB9B 507219EE 95DB113A 917678B2 73BED6B8 E3C1743B 7116E69E 22229516 3FF1CAA1 681FAC09 120ECA30 7586E1A7"
    );

    cbc_test!(
        test_cbc_aes128_dec,
        CbcDecryption<aes::Aes>,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "7649ABAC 8119B246 CEE98E9B 12E9197D 5086CB9B 507219EE 95DB113A 917678B2 73BED6B8 E3C1743B 7116E69E 22229516 3FF1CAA1 681FAC09 120ECA30 7586E1A7",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

    cbc_test!(
        test_cbc_aes192_enc,
        CbcEncryption<aes::Aes>,
        "8E73B0F7 DA0E6452 C810F32B 809079E5 62F8EAD2 522C6B7B",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "4F021DB2 43BC633D 7178183A 9FA071E8 B4D9ADA9 AD7DEDF4 E5E73876 3F69145A 571B2420 12FB7AE0 7FA9BAAC 3DF102E0 08B0E279 88598881 D920A9E6 4F5615CD"
    );

    cbc_test!(
        test_cbc_aes192_dec,
        CbcDecryption<aes::Aes>,
        "8E73B0F7 DA0E6452 C810F32B 809079E5 62F8EAD2 522C6B7B",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "4F021DB2 43BC633D 7178183A 9FA071E8 B4D9ADA9 AD7DEDF4 E5E73876 3F69145A 571B2420 12FB7AE0 7FA9BAAC 3DF102E0 08B0E279 88598881 D920A9E6 4F5615CD",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

    cbc_test!(
        test_cbc_aes256_enc,
        CbcEncryption<aes::Aes>,
        "603DEB10 15CA71BE 2B73AEF0 857D7781 1F352C07 3B6108D7 2D9810A3 0914DFF4",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "F58C4C04 D6E5F1BA 779EABFB 5F7BFBD6 9CFC4E96 7EDB808D 679F777B C6702C7D 39F23369 A9D9BACF A530E263 04231461 B2EB05E2 C39BE9FC DA6C1907 8C6A9D1B"
    );

    cbc_test!(
        test_cbc_aes256_dec,
        CbcDecryption<aes::Aes>,
        "603DEB10 15CA71BE 2B73AEF0 857D7781 1F352C07 3B6108D7 2D9810A3 0914DFF4",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "F58C4C04 D6E5F1BA 779EABFB 5F7BFBD6 9CFC4E96 7EDB808D 679F777B C6702C7D 39F23369 A9D9BACF A530E263 04231461 B2EB05E2 C39BE9FC DA6C1907 8C6A9D1B",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );    
}
