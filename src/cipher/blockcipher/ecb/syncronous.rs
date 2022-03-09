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

/// ECB encryption provider
/// 
/// Provides encryption in Electronic Codebook Mode based on a Primitive T eg. Aes
pub struct EcbEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<T: PrimitiveEncryption<B>, const B: usize> EcbEncryption<T, B> {

    /// Create a new instance from a Cipher primitive
    pub fn new(primitive: T) -> Self {
        Self { 
            primitive,
            buffer: FixedBuffer::new(),
            out: Vec::new(),
        }
    }

    fn process_buffer(&mut self) {

        // Encrypt the buffer
        self.primitive.encrypt(self.buffer.as_mut(), None, None);

        // Extract the encrypted buffer and replace it with a fresh one
        let encrypted = mem::replace(&mut self.buffer, FixedBuffer::new());

        // Append the extracted buffer to out
        self.out.extend(encrypted);
    }

    /// Resets the cipher and returns a Readable with the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {

        // If the last block is complete then encrypt, else return error with number of missing bytes
        if self.buffer.is_full() { self.process_buffer(); }
       
        else if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( mem::replace(&mut self.out, Vec::new()) ))
    }
}

impl<T: PrimitiveEncryption<B>, const B: usize> io::Write for EcbEncryption<T, B> {

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

/// ECB decryption provider
/// 
/// Provides decryption in Electronic Codebook Mode based on a Primitive T eg. Aes
pub struct EcbDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    primitive: T,
    buffer: FixedBuffer<u8, BLOCKSIZE>,
    out: Vec<u8>
}

impl<T: PrimitiveDecryption<B>, const B: usize> EcbDecryption<T, B> {

    /// Create a new instance from a Cipher primitive
    pub fn new(primitive: T) -> Self {
        Self { 
            primitive,
            buffer: FixedBuffer::new(),
            out: Vec::new()
        }
    }

    fn process_buffer(&mut self) {

        // Encrypt the buffer
        self.primitive.decrypt(self.buffer.as_mut(), None, None);

        // Extract the encrypted buffer and replace it with a fresh one
        let decrypted = mem::replace(&mut self.buffer, FixedBuffer::new());

        // Append the extracted buffer to out
        self.out.extend(decrypted);
    }

    /// Resets the cipher and returns a Readable with the processed contents
    pub fn finalize(&mut self) -> Result<Readable<Vec<u8>>, BlockCipherError> {
        // If the last block is complete then encrypt, else return error with number of missing bytes
        if self.buffer.is_full() { self.process_buffer(); }
    
        else if !self.buffer.is_full() { return Err( BlockCipherError::IncompleteBlock( self.buffer.capacity() ) ) }

        // Replace out with a fresh vec and return a readable with the contents of out
        Ok( Readable::new( std::mem::replace(&mut self.out, Vec::new()) ))
    }
}

impl<T: PrimitiveDecryption<B>, const B: usize> io::Write for EcbDecryption<T, B> {

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

    macro_rules! ecb_test_enc {
        (
            $fn_name: ident,
            $primitive: ty,
            $key: literal,
            $input: literal,
            $expected: literal
        ) => {

            #[test]
            fn $fn_name() {

                let input = decode($input);
                let key = decode($key);
                let expected = decode($expected);
                let mut output = Vec::new();
                
                let mut cipher = <$primitive>::new(&key).with_ecb_encryption();
                cipher.write_all(&input).unwrap();
                let mut reader = cipher.finalize().unwrap();
                reader.read_to_end(&mut output).unwrap();

                assert_eq!(expected, output);
            }
            
        };
    }

    macro_rules! ecb_test_dec {
        (
            $fn_name: ident,
            $primitive: ty,
            $key: literal,
            $input: literal,
            $expected: literal
        ) => {

            #[test]
            fn $fn_name() {

                let input = decode($input);
                let key = decode($key);
                let expected = decode($expected);
                let mut output = Vec::new();
                
                let mut cipher = <$primitive>::new(&key).with_ecb_decryption();
                cipher.write_all(&input).unwrap();
                let mut reader = cipher.finalize().unwrap();
                reader.read_to_end(&mut output).unwrap();

                assert_eq!(expected, output);
            }
        };
    }

    // Example values from [NIST](https://csrc.nist.gov/projects/cryptographic-standards-and-guidelines/example-values)

    ecb_test_enc!(
        test_ecb_aes128_enc,
        aes::Aes,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "3AD77BB4 0D7A3660 A89ECAF3 2466EF97 F5D3D585 03B9699D E785895A 96FDBAAF 43B1CD7F 598ECE23 881B00E3 ED030688 7B0C785E 27E8AD3F 82232071 04725DD4"
    );

    ecb_test_dec!(
        test_ecb_aes128_dec,
        aes::Aes,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        "3AD77BB4 0D7A3660 A89ECAF3 2466EF97 F5D3D585 03B9699D E785895A 96FDBAAF 43B1CD7F 598ECE23 881B00E3 ED030688 7B0C785E 27E8AD3F 82232071 04725DD4",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

    ecb_test_enc!(
        test_ecb_aes192_enc,
        aes::Aes,
        "8E73B0F7 DA0E6452 C810F32B 809079E5 62F8EAD2 522C6B7B",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "BD334F1D 6E45F25F F712A214 571FA5CC 97410484 6D0AD3AD 7734ECB3 ECEE4EEF EF7AFD22 70E2E60A DCE0BA2F ACE6444E 9A4B41BA 738D6C72 FB166916 03C18E0E"
    );

    ecb_test_dec!(
        test_ecb_aes192_dec,
        aes::Aes,
        "8E73B0F7 DA0E6452 C810F32B 809079E5 62F8EAD2 522C6B7B",
        "BD334F1D 6E45F25F F712A214 571FA5CC 97410484 6D0AD3AD 7734ECB3 ECEE4EEF EF7AFD22 70E2E60A DCE0BA2F ACE6444E 9A4B41BA 738D6C72 FB166916 03C18E0E",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

    ecb_test_enc!(
        test_ecb_aes256_enc,
        aes::Aes,
        "603DEB10 15CA71BE 2B73AEF0 857D7781 1F352C07 3B6108D7 2D9810A3 0914DFF4",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "F3EED1BD B5D2A03C 064B5A7E 3DB181F8 591CCB10 D410ED26 DC5BA74A 31362870 B6ED21B9 9CA6F4F9 F153E7B1 BEAFED1D 23304B7A 39F9F3FF 067D8D8F 9E24ECC7"
    );

    ecb_test_dec!(
        test_ecb_aes256_dec,
        aes::Aes,
        "603DEB10 15CA71BE 2B73AEF0 857D7781 1F352C07 3B6108D7 2D9810A3 0914DFF4",
        "F3EED1BD B5D2A03C 064B5A7E 3DB181F8 591CCB10 D410ED26 DC5BA74A 31362870 B6ED21B9 9CA6F4F9 F153E7B1 BEAFED1D 23304B7A 39F9F3FF 067D8D8F 9E24ECC7",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );
}