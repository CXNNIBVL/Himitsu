mod common;

#[cfg(test)]
mod tests {

    use super::common::{decode, decode_into_array};
    use himitsu::{
        cipher::block::primitive::aes::*,
        traits::cipher::primitive::{
            BlockCipherPrimitiveDecryption, BlockCipherPrimitiveEncryption,
        },
    };

    #[test]
    fn test_aes_128_enc() {
        let plaintext = decode("00112233445566778899aabbccddeeff");
        let key = decode_into_array::<AES_128_KEYLEN>("000102030405060708090a0b0c0d0e0f");
        let expected = decode("69c4e0d86a7b0430d8cdb78070b4c55a");

        let mut buf = [0; AES_BLOCKSIZE];

        for (i, element) in plaintext.into_iter().enumerate() {
            buf[i] = element;
        }

        let aes = Aes128::new(key);
        aes.encrypt(&mut buf);

        assert_eq!(expected, buf.as_ref());
    }

    #[test]
    fn test_aes_192_enc() {
        let plaintext = decode("00112233445566778899aabbccddeeff");
        let key =
            decode_into_array::<AES_192_KEYLEN>("000102030405060708090a0b0c0d0e0f1011121314151617");
        let expected = decode("dda97ca4864cdfe06eaf70a0ec0d7191");

        let mut buf = [0; AES_BLOCKSIZE];

        for (i, element) in plaintext.into_iter().enumerate() {
            buf[i] = element;
        }

        let aes = Aes192::new(key);
        aes.encrypt(&mut buf);

        assert_eq!(expected, buf.as_ref());
    }

    #[test]
    fn test_aes_256_enc() {
        let plaintext = decode("00112233445566778899aabbccddeeff");
        let key = decode_into_array::<AES_256_KEYLEN>(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        );
        let expected = decode("8ea2b7ca516745bfeafc49904b496089");

        let mut buf = [0; AES_BLOCKSIZE];

        for (i, element) in plaintext.into_iter().enumerate() {
            buf[i] = element;
        }

        let aes = Aes256::new(key);
        aes.encrypt(&mut buf);

        assert_eq!(expected, buf.as_ref());
    }

    #[test]
    fn test_aes_128_dec() {
        let plaintext = decode("69c4e0d86a7b0430d8cdb78070b4c55a");
        let key = decode_into_array::<AES_128_KEYLEN>("000102030405060708090a0b0c0d0e0f");
        let expected = decode("00112233445566778899aabbccddeeff");

        let mut buf = [0; AES_BLOCKSIZE];

        for (i, element) in plaintext.into_iter().enumerate() {
            buf[i] = element;
        }

        let aes = Aes128::new(key);
        aes.decrypt(&mut buf);

        assert_eq!(expected, buf.as_ref());
    }
}
