use std::io;
use std::mem;
use crate::errors::blockcipher::BlockCipherError;
use crate::util::{
    readable::Readable,
    buffer::FixedBuffer
};
use crate::traits::cipher::{ 
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
};

/// CBC Encryption Provider
pub struct CbcEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    iv: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<T: PrimitiveEncryption<B>, const B: usize> CbcEncryption<T, B> {

    /// Create a new CBC Encryption instance from a primitive and an IV.
    /// Up to the primitives blocksize of IV contents will be used.
    pub fn new(primitive: T, iv: &[u8]) -> Self {

        let ( buffer, mut iv_buf ) = ( FixedBuffer::new(), FixedBuffer::new() );
        iv_buf.push_slice(iv);

        let out = Vec::new();
        
        Self { primitive, buffer, iv: iv_buf, out }
    }

    fn process_buffer(&mut self) {
        self.primitive.encrypt(self.buffer.as_mut(), Some(self.iv.as_ref()), None);
        let encrypted = mem::replace(&mut self.buffer, FixedBuffer::new());
        
        self.iv.override_contents(encrypted.as_ref(), encrypted.len());
        self.out.extend(encrypted);
    }

    fn process_final(&mut self) {
        self.primitive.encrypt(self.buffer.as_mut(), Some(self.iv.as_ref()), None);
        let encrypted = mem::replace(&mut self.buffer, FixedBuffer::new());
        
        self.out.extend(encrypted);
    }

    /// Resets the cipher and returns a Readable with the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        // If the last block is complete then encrypt
        if self.buffer.is_full() { self.process_final(); }
        // Else return error with number of missing bytes
        else if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( mem::replace(&mut self.out, Vec::new()) ))
    }

    /// Resets the cipher, as well as the underlying IV and returns a Readable with the processed contents
    pub fn finalize_with_iv(&mut self, iv: &[u8]) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        let out = self.finalize();

        self.iv = {
            let mut new_iv = FixedBuffer::new();
            new_iv.push_slice(iv);
            new_iv
        };

        out
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> io::Write for CbcEncryption<T, B> {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        // Push buf until all contents have been written, if necessary, then encrypt buffer
        while written < buf.len() {

            if self.buffer.is_full() { self.process_buffer(); }

            written += self.buffer.push_slice(&buf[written..]);
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    
}


pub struct CbcDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    iv: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<T: PrimitiveDecryption<B>, const B: usize> CbcDecryption<T, B> {

    pub fn new(primitive: T, iv: &[u8]) -> Self {

        let ( buffer, mut iv_buf ) = ( FixedBuffer::new(), FixedBuffer::new() );
        iv_buf.push_slice(iv);

        let out = Vec::new();
        
        Self { primitive, buffer, iv: iv_buf, out}
    }

    fn process_buffer(&mut self) {

        let new_iv = FixedBuffer::from(self.buffer.as_ref()); 
    
        self.primitive.decrypt(self.buffer.as_mut(), None, Some(self.iv.as_ref()));
        let decrypted = mem::replace(&mut self.buffer, FixedBuffer::new());

        self.iv = new_iv;
        
        self.out.extend(decrypted);
    }

    fn process_final(&mut self) {
        
        self.primitive.decrypt(self.buffer.as_mut(), None, Some(self.iv.as_ref()));
        let decrypted = mem::replace(&mut self.buffer, FixedBuffer::new());
        
        self.out.extend(decrypted);
    }

    /// Resets the cipher and returns a Readable with the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        // If the last block is complete then encrypt
        if self.buffer.is_full() { self.process_final(); }
        // Else return error with number of missing bytes
        else if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( mem::replace(&mut self.out, Vec::new()) ))
    }

    /// Resets the cipher, as well as the underlying IV and returns a Readable with the processed contents
    pub fn finalize_with_iv(&mut self, iv: &[u8]) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        let out = self.finalize();

        self.iv = {
            let mut new_iv = FixedBuffer::new();
            new_iv.push_slice(iv);
            new_iv
        };

        out
    }
}

impl<T: PrimitiveDecryption<B>, const B: usize> io::Write for CbcDecryption<T, B> {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        // Push buf until all contents have been written, if necessary, then encrypt buffer
        while written < buf.len() {

            if self.buffer.is_full() { self.process_buffer(); }

            written += self.buffer.push_slice(&buf[written..]);
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {

    use std::io::{Read, Write};
    use crate::cipher::blockcipher::primitive::aes;
    use crate::traits::cipher::*;

    fn decode(s: &str) -> Vec<u8> {
		use crate::encode::HexEncoder;
		HexEncoder::builder().decode(s)
	}

    macro_rules! cbc_test_enc {
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

                let mut cipher = <$cipher>::new(&key).with_cbc_encryption(&iv);
                cipher.write_all(&input).unwrap();

                let mut reader = cipher.finalize().unwrap();

                let mut output = Vec::new();
                reader.read_to_end(&mut output).unwrap();

                assert_eq!(expected, output);

            }
            
        };
    }

    macro_rules! cbc_test_dec {
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

                let mut cipher = <$cipher>::new(&key).with_cbc_decryption(&iv);
                cipher.write_all(&input).unwrap();

                let mut reader = cipher.finalize().unwrap();

                let mut output = Vec::new();
                reader.read_to_end(&mut output).unwrap();

                assert_eq!(expected, output);

            }
            
        };
    }

    // Example values from [NIST](https://csrc.nist.gov/projects/cryptographic-standards-and-guidelines/example-values)

    cbc_test_enc!(
        test_cbc_aes128_enc,
        aes::Aes,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "7649ABAC 8119B246 CEE98E9B 12E9197D 5086CB9B 507219EE 95DB113A 917678B2 73BED6B8 E3C1743B 7116E69E 22229516 3FF1CAA1 681FAC09 120ECA30 7586E1A7"
    );

    cbc_test_dec!(
        test_cbc_aes128_dec,
        aes::Aes,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "7649ABAC 8119B246 CEE98E9B 12E9197D 5086CB9B 507219EE 95DB113A 917678B2 73BED6B8 E3C1743B 7116E69E 22229516 3FF1CAA1 681FAC09 120ECA30 7586E1A7",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

    cbc_test_enc!(
        test_cbc_aes192_enc,
        aes::Aes,
        "8E73B0F7 DA0E6452 C810F32B 809079E5 62F8EAD2 522C6B7B",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "4F021DB2 43BC633D 7178183A 9FA071E8 B4D9ADA9 AD7DEDF4 E5E73876 3F69145A 571B2420 12FB7AE0 7FA9BAAC 3DF102E0 08B0E279 88598881 D920A9E6 4F5615CD"
    );

    cbc_test_dec!(
        test_cbc_aes192_dec,
        aes::Aes,
        "8E73B0F7 DA0E6452 C810F32B 809079E5 62F8EAD2 522C6B7B",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "4F021DB2 43BC633D 7178183A 9FA071E8 B4D9ADA9 AD7DEDF4 E5E73876 3F69145A 571B2420 12FB7AE0 7FA9BAAC 3DF102E0 08B0E279 88598881 D920A9E6 4F5615CD",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

    cbc_test_enc!(
        test_cbc_aes256_enc,
        aes::Aes,
        "603DEB10 15CA71BE 2B73AEF0 857D7781 1F352C07 3B6108D7 2D9810A3 0914DFF4",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "F58C4C04 D6E5F1BA 779EABFB 5F7BFBD6 9CFC4E96 7EDB808D 679F777B C6702C7D 39F23369 A9D9BACF A530E263 04231461 B2EB05E2 C39BE9FC DA6C1907 8C6A9D1B"
    );

    cbc_test_dec!(
        test_cbc_aes256_dec,
        aes::Aes,
        "603DEB10 15CA71BE 2B73AEF0 857D7781 1F352C07 3B6108D7 2D9810A3 0914DFF4",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "F58C4C04 D6E5F1BA 779EABFB 5F7BFBD6 9CFC4E96 7EDB808D 679F777B C6702C7D 39F23369 A9D9BACF A530E263 04231461 B2EB05E2 C39BE9FC DA6C1907 8C6A9D1B",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );    
}
