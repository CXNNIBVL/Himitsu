mod common;

#[cfg(test)]
mod tests {
    use super::common::{decode, decode_into_array};
    use himitsu::cipher::block::{
        primitive::aes,
        ecb::{
            ThreadedEcbEncryptionProvider, 
            ThreadedEcbDecryptionProvider
        }
    };
    use std::io::Write;

    macro_rules! ecb_test_enc {
        (
            $fn_name: ident,
            $primitive: ty,
            $key: literal,
            $keylen: expr,
            $input: literal,
            $expected: literal
        ) => {
            #[test]
            fn $fn_name() {
                let input = decode($input);
                let key = decode_into_array::<$keylen>($key);
                let expected = decode($expected);

                let mut cipher = <$primitive>::new(key).with_threaded_ecb_encryption(4);
                cipher.write_all(&input).unwrap();
                let output: Vec<u8> = cipher.finalize();

                assert_eq!(expected, output);
            }
        };
    }

    macro_rules! ecb_test_dec {
        (
            $fn_name: ident,
            $primitive: ty,
            $key: literal,
            $keylen: expr,
            $input: literal,
            $expected: literal
        ) => {
            #[test]
            fn $fn_name() {
                let input = decode($input);
                let key = decode_into_array::<$keylen>($key);
                let expected = decode($expected);

                let mut cipher = <$primitive>::new(key).with_threaded_ecb_decryption(4);
                cipher.write_all(&input).unwrap();
                let output: Vec<u8> = cipher.finalize();

                assert_eq!(expected, output);
            }
        };
    }

    // Example values from [NIST](https://csrc.nist.gov/projects/cryptographic-standards-and-guidelines/example-values)

    ecb_test_enc!(
        test_threaded_ecb_aes128_enc,
        aes::Aes128,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        {aes::AES_128_KEYLEN},
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "3AD77BB4 0D7A3660 A89ECAF3 2466EF97 F5D3D585 03B9699D E785895A 96FDBAAF 43B1CD7F 598ECE23 881B00E3 ED030688 7B0C785E 27E8AD3F 82232071 04725DD4"
    );

    ecb_test_dec!(
        test_threaded_ecb_aes128_dec,
        aes::Aes128,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        {aes::AES_128_KEYLEN},
        "3AD77BB4 0D7A3660 A89ECAF3 2466EF97 F5D3D585 03B9699D E785895A 96FDBAAF 43B1CD7F 598ECE23 881B00E3 ED030688 7B0C785E 27E8AD3F 82232071 04725DD4",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

    ecb_test_enc!(
        test_threaded_ecb_aes192_enc,
        aes::Aes192,
        "8E73B0F7 DA0E6452 C810F32B 809079E5 62F8EAD2 522C6B7B",
        {aes::AES_192_KEYLEN},
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "BD334F1D 6E45F25F F712A214 571FA5CC 97410484 6D0AD3AD 7734ECB3 ECEE4EEF EF7AFD22 70E2E60A DCE0BA2F ACE6444E 9A4B41BA 738D6C72 FB166916 03C18E0E"
    );

    ecb_test_dec!(
        test_threaded_ecb_aes192_dec,
        aes::Aes192,
        "8E73B0F7 DA0E6452 C810F32B 809079E5 62F8EAD2 522C6B7B",
        {aes::AES_192_KEYLEN},
        "BD334F1D 6E45F25F F712A214 571FA5CC 97410484 6D0AD3AD 7734ECB3 ECEE4EEF EF7AFD22 70E2E60A DCE0BA2F ACE6444E 9A4B41BA 738D6C72 FB166916 03C18E0E",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

    ecb_test_enc!(
        test_threaded_ecb_aes256_enc,
        aes::Aes256,
        "603DEB10 15CA71BE 2B73AEF0 857D7781 1F352C07 3B6108D7 2D9810A3 0914DFF4",
        {aes::AES_256_KEYLEN},
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "F3EED1BD B5D2A03C 064B5A7E 3DB181F8 591CCB10 D410ED26 DC5BA74A 31362870 B6ED21B9 9CA6F4F9 F153E7B1 BEAFED1D 23304B7A 39F9F3FF 067D8D8F 9E24ECC7"
    );

    ecb_test_dec!(
        test_threaded_ecb_aes256_dec,
        aes::Aes256,
        "603DEB10 15CA71BE 2B73AEF0 857D7781 1F352C07 3B6108D7 2D9810A3 0914DFF4",
        {aes::AES_256_KEYLEN},
        "F3EED1BD B5D2A03C 064B5A7E 3DB181F8 591CCB10 D410ED26 DC5BA74A 31362870 B6ED21B9 9CA6F4F9 F153E7B1 BEAFED1D 23304B7A 39F9F3FF 067D8D8F 9E24ECC7",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );
}
