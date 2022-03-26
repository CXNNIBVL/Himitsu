mod common;

#[cfg(test)]
mod tests {

    use super::common::{decode, decode_into_array};
    use himitsu::prelude::*;
    use himitsu::cipher::{
        block::primitive::aes,
        stream::cfb::{CfbEncryptionProvider, CfbDecryptionProvider}
    };

    macro_rules! cfb_test_enc {
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
                let mut input = decode($input);
                let key = decode($key);
                let iv = decode_into_array($iv);
                let expected = decode($expected);

                let mut cipher = <$cipher>::new(&key).with_cfb_encryption(iv);
                cipher.encrypt(&mut input);

                assert_eq!(expected, input);
            }
        };
    }

    macro_rules! cfb_test_dec {
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
                let mut input = decode($input);
                let key = decode($key);
                let iv = decode_into_array($iv);
                let expected = decode($expected);

                let mut cipher = <$cipher>::new(&key).with_cfb_decryption(iv);
                cipher.decrypt(&mut input);

                assert_eq!(expected, input);
            }
        };
    }

    // Example values from [NIST](https://csrc.nist.gov/projects/cryptographic-standards-and-guidelines/example-values)

    cfb_test_enc!(
        test_cfb_aes128_enc,
        aes::Aes,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710",
        "3B3FD92E B72DAD20 333449F8 E83CFB4A C8A64537 A0B3A93F CDE3CDAD 9F1CE58B 26751F67 A3CBB140 B1808CF1 87A4F4DF C04B0535 7C5D1C0E EAC4C66F 9FF7F2E6"
    );

    cfb_test_dec!(
        test_cfb_aes128_dec,
        aes::Aes,
        "2B7E1516 28AED2A6 ABF71588 09CF4F3C",
        "00010203 04050607 08090A0B 0C0D0E0F",
        "3B3FD92E B72DAD20 333449F8 E83CFB4A C8A64537 A0B3A93F CDE3CDAD 9F1CE58B 26751F67 A3CBB140 B1808CF1 87A4F4DF C04B0535 7C5D1C0E EAC4C66F 9FF7F2E6",
        "6BC1BEE2 2E409F96 E93D7E11 7393172A AE2D8A57 1E03AC9C 9EB76FAC 45AF8E51 30C81C46 A35CE411 E5FBC119 1A0A52EF F69F2445 DF4F9B17 AD2B417B E66C3710"
    );

}
