use crate::blockcipher::{
    BlockCipherEncryption,
    BlockCipherDecryption,
};
use crate::{util, util::Array, array};
use super::constants::*;

pub type Aes128 = Aes<AES_128_KEYLEN, AES_128_EXPANDED_KEYLEN>;
pub type Aes192 = Aes<AES_192_KEYLEN, AES_192_EXPANDED_KEYLEN>;
pub type Aes256 = Aes<AES_256_KEYLEN, AES_256_EXPANDED_KEYLEN>;

fn key_expansion_gcon(k: &mut [u8; 4]) {
    // Apply S_BOX
    k[0] = sbox(k[0] as usize);
    k[1] = sbox(k[1] as usize);
    k[2] = sbox(k[2] as usize);
    k[3] = sbox(k[3] as usize);
}

fn key_expansion_rcon(k: &mut [u8; 4], iteration: usize) {
    k.rotate_left(1);
    key_expansion_gcon(k);
    k[0] ^= rcon(iteration);
}

fn key_expansion<const IN_LEN: usize, const OUT_LEN: usize>(
    key: Array<u8, IN_LEN>,
) -> Array<u8, OUT_LEN> {
    let mut expanded_key: Array<u8, OUT_LEN> = Array::default();
    let mut bytes_generated = 0;

    for by in key {
        expanded_key[bytes_generated] = by;
        bytes_generated += 1;
    }

    let mut rcon_iteration = 1;
    let mut tmp: Array<u8, 4> = Array::default();

    while bytes_generated < OUT_LEN {
        tmp.copy_from_slice(&expanded_key[bytes_generated - 4..bytes_generated]);

        if bytes_generated % 16 == 0
            && bytes_generated % 32 != 0
            && OUT_LEN == AES_256_EXPANDED_KEYLEN
        {
            key_expansion_gcon(&mut tmp);
        }

        if bytes_generated % IN_LEN == 0 {
            key_expansion_rcon(&mut tmp, rcon_iteration);
            rcon_iteration += 1;
        }

        let ix = bytes_generated - IN_LEN;
        util::xor_buffers(tmp.as_mut(), &expanded_key[ix..ix + 4]);

        for i in 0..4 {
            expanded_key[bytes_generated] = tmp[i];
            bytes_generated += 1;
        }
    }

    expanded_key
}

/// Aes Encryption and Decryption provider
pub struct Aes<const KEY_INPUT_LEN: usize, const KEY_EXPANDED_LEN: usize> {
    rounds: usize,
    key: Array<u8, KEY_EXPANDED_LEN>,
}

impl<const KI: usize, const KE: usize> Aes<KI, KE> {
    pub fn new(key: [u8; KI]) -> Self {

        let expanded_key = key_expansion( array!(key) );

        Self {
            rounds: (KE / AES_BLOCKSIZE) - 1,
            key: expanded_key
        }
    }
}

// Returns start, end of keyindex, of a given round
fn key_index(round: usize) -> (usize, usize) {
    let start = round * AES_BLOCKSIZE;
    let end = start + AES_BLOCKSIZE;

    (start, end)
}

fn roundkey(expanded_key: &[u8], round: usize) -> &[u8] {
    let (start, end) = key_index(round);
    &expanded_key[start..end]
}

impl<const KI: usize, const KE: usize> BlockCipherEncryption<AES_BLOCKSIZE> for Aes<KI, KE> {
    fn encrypt(&mut self, state: &mut Block) {
        add_roundkey(state.as_mut(), roundkey(self.key.as_ref(), 0));

        for round in 1..self.rounds {
            sub_bytes_enc(state.as_mut());
            shift_rows_enc(state.as_mut());
            mix_columns_enc(state.as_mut());

            add_roundkey(state.as_mut(), roundkey(self.key.as_ref(), round));
        }

        sub_bytes_enc(state.as_mut());
        shift_rows_enc(state.as_mut());

        add_roundkey(state.as_mut(), roundkey(self.key.as_ref(), self.rounds));
    }
}

impl<const KI: usize, const KE: usize> BlockCipherDecryption<AES_BLOCKSIZE> for Aes<KI, KE> {
    fn decrypt(&mut self, state: &mut Block) {
        add_roundkey(state.as_mut(), roundkey(self.key.as_ref(), self.rounds));
        sub_bytes_dec(state.as_mut());
        shift_rows_dec(state.as_mut());

        for round in (1..self.rounds).rev() {
            add_roundkey(state.as_mut(), roundkey(self.key.as_ref(), round));

            mix_columns_dec(state.as_mut());
            sub_bytes_dec(state.as_mut());
            shift_rows_dec(state.as_mut());
        }

        add_roundkey(state.as_mut(), roundkey(self.key.as_ref(), 0));
    }
}

/// Xor round key into state
fn add_roundkey(state: &mut [u8], key: &[u8]) {
    util::xor_buffers(state, key);
}

/// Substitute with SBOX
fn sub_bytes_enc(state: &mut [u8]) {
    for by in state {
        *by = sbox(*by as usize);
    }
}

fn shift_rows_enc(state: &mut [u8]) {
    // Row 1	a b c d -> b c d a
    state.swap(1, 5);
    state.swap(5, 13);
    state.swap(5, 9);

    // Row 2	a b c d -> c d a b
    state.swap(2, 10);
    state.swap(6, 14);

    // Row 3	a b c d -> d a b c
    state.swap(3, 15);
    state.swap(7, 15);
    state.swap(11, 15);
}

fn mix_columns_enc(state: &mut [u8]) {
    let mut tmp: Array<u8, 4> = Array::default();

    for i in 0..4 {
        let ix = i * 4;

        tmp[0] = mul2(state[ix] as usize) ^ mul3(state[ix + 1] as usize) ^ state[ix + 2] ^ state[ix + 3];
        tmp[1] = state[ix] ^ mul2(state[ix + 1] as usize) ^ mul3(state[ix + 2] as usize) ^ state[ix + 3];
        tmp[2] = state[ix] ^ state[ix + 1] ^ mul2(state[ix + 2] as usize) ^ mul3(state[ix + 3] as usize);
        tmp[3] = mul3(state[ix] as usize) ^ state[ix + 1] ^ state[ix + 2] ^ mul2(state[ix + 3] as usize);

        state[ix] = tmp[0];
        state[ix + 1] = tmp[1];
        state[ix + 2] = tmp[2];
        state[ix + 3] = tmp[3];
    }
}

fn sub_bytes_dec(state: &mut [u8]) {
    for by in state {
        *by = inv_sbox(*by as usize);
    }
}

fn shift_rows_dec(state: &mut [u8]) {
    // Row 1	b c d a -> a b c d
    state.swap(1, 13);
    state.swap(5, 13);
    state.swap(9, 13);

    // Row 2	c d a b -> a b c d
    state.swap(2, 10);
    state.swap(6, 14);

    // Row 3	d a b c -> a b c d
    state.swap(3, 7);
    state.swap(7, 11);
    state.swap(11, 15);
}

fn mix_columns_dec(state: &mut [u8]) {
    let mut tmp: Array<u8, 4> = Array::default();

    for i in 0..4 {
        let ix = i * 4;

        tmp[0] = mul14(state[ix] as usize) ^ mul11(state[ix + 1] as usize) ^ mul13(state[ix + 2] as usize) ^ mul9(state[ix + 3] as usize);
        tmp[1] = mul9(state[ix] as usize) ^ mul14(state[ix + 1] as usize) ^ mul11(state[ix + 2] as usize) ^ mul13(state[ix + 3] as usize);
        tmp[2] = mul13(state[ix] as usize) ^ mul9(state[ix + 1] as usize) ^ mul14(state[ix + 2] as usize) ^ mul11(state[ix + 3] as usize);
        tmp[3] = mul11(state[ix] as usize) ^ mul13(state[ix + 1] as usize) ^ mul9(state[ix + 2] as usize) ^ mul14(state[ix + 3] as usize);
    
        state[ix] = tmp[0];
        state[ix + 1] = tmp[1];
        state[ix + 2] = tmp[2];
        state[ix + 3] = tmp[3];
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn decode(s: &str) -> Vec<u8> {
        use crate::encode::hex::HexDecoder;
        s.chars().decode_hex()
    }

    fn decode_into_array<const B: usize>(s: &str) -> [u8; B] {
        decode(s).try_into().unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", B, v.len())
        })
    }

    #[test]
    fn test_key_expansion_16byte() {
        let key_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";
        let expected_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		62 63 63 63 62 63 63 63 62 63 63 63 62 63 63 63 
		9b 98 98 c9 f9 fb fb aa 9b 98 98 c9 f9 fb fb aa 
		90 97 34 50 69 6c cf fa f2 f4 57 33 0b 0f ac 99 
		ee 06 da 7b 87 6a 15 81 75 9e 42 b2 7e 91 ee 2b 
		7f 2e 2b 88 f8 44 3e 09 8d da 7c bb f3 4b 92 90 
		ec 61 4b 85 14 25 75 8c 99 ff 09 37 6a b4 9b a7 
		21 75 17 87 35 50 62 0b ac af 6b 3c c6 1b f0 9b 
		0e f9 03 33 3b a9 61 38 97 06 0a 04 51 1d fa 9f 
		b1 d4 d8 e2 8a 7d b9 da 1d 7b b3 de 4c 66 49 41 
		b4 ef 5b cb 3e 92 e2 11 23 e9 51 cf 6f 8f 18 8e";

        let (key, expected) = (
            decode_into_array::<AES_128_KEYLEN>(key_str),
            decode(expected_str),
        );
        let expanded: Array<u8, AES_128_EXPANDED_KEYLEN> = key_expansion( array!(key) );

        assert_eq!(expanded.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_key_expansion_24byte() {
        let key_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";
        let expected_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		00 00 00 00 00 00 00 00 62 63 63 63 62 63 63 63 
		62 63 63 63 62 63 63 63 62 63 63 63 62 63 63 63 
		9b 98 98 c9 f9 fb fb aa 9b 98 98 c9 f9 fb fb aa 
		9b 98 98 c9 f9 fb fb aa 90 97 34 50 69 6c cf fa 
		f2 f4 57 33 0b 0f ac 99 90 97 34 50 69 6c cf fa 
		c8 1d 19 a9 a1 71 d6 53 53 85 81 60 58 8a 2d f9 
		c8 1d 19 a9 a1 71 d6 53 7b eb f4 9b da 9a 22 c8 
		89 1f a3 a8 d1 95 8e 51 19 88 97 f8 b8 f9 41 ab 
		c2 68 96 f7 18 f2 b4 3f 91 ed 17 97 40 78 99 c6 
		59 f0 0e 3e e1 09 4f 95 83 ec bc 0f 9b 1e 08 30 
		0a f3 1f a7 4a 8b 86 61 13 7b 88 5f f2 72 c7 ca 
		43 2a c8 86 d8 34 c0 b6 d2 c7 df 11 98 4c 59 70";

        let (key, expected) = (
            decode_into_array::<AES_192_KEYLEN>(key_str),
            decode(expected_str),
        );
        let expanded: Array<u8, AES_192_EXPANDED_KEYLEN> = key_expansion(array!(key));

        assert_eq!(expanded.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_key_expansion_32byte() {
        let key_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";
        let expected_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		62 63 63 63 62 63 63 63 62 63 63 63 62 63 63 63 
		aa fb fb fb aa fb fb fb aa fb fb fb aa fb fb fb 
		6f 6c 6c cf 0d 0f 0f ac 6f 6c 6c cf 0d 0f 0f ac 
		7d 8d 8d 6a d7 76 76 91 7d 8d 8d 6a d7 76 76 91 
		53 54 ed c1 5e 5b e2 6d 31 37 8e a2 3c 38 81 0e 
		96 8a 81 c1 41 fc f7 50 3c 71 7a 3a eb 07 0c ab 
		9e aa 8f 28 c0 f1 6d 45 f1 c6 e3 e7 cd fe 62 e9 
		2b 31 2b df 6a cd dc 8f 56 bc a6 b5 bd bb aa 1e 
		64 06 fd 52 a4 f7 90 17 55 31 73 f0 98 cf 11 19 
		6d bb a9 0b 07 76 75 84 51 ca d3 31 ec 71 79 2f 
		e7 b0 e8 9c 43 47 78 8b 16 76 0b 7b 8e b9 1a 62 
		74 ed 0b a1 73 9b 7e 25 22 51 ad 14 ce 20 d4 3b 
		10 f8 0a 17 53 bf 72 9c 45 c9 79 e7 cb 70 63 85 ";

        let (key, expected) = (
            decode_into_array::<AES_256_KEYLEN>(key_str),
            decode(expected_str),
        );
        let expanded: Array<u8, AES_256_EXPANDED_KEYLEN> = key_expansion(array!(key));

        assert_eq!(expanded.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_add_roundkey() {
        let key = decode("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f");
        let mut state = decode("00 11 22 33 44 55 66 77 88 99 aa bb cc dd ee ff");
        let expected = decode("00102030405060708090a0b0c0d0e0f0");

        add_roundkey(&mut state, &key);

        assert_eq!(expected, state)
    }

    #[test]
    fn test_sub_bytes_enc() {
        let mut state = decode("00 10 20 30 40 50 60 70 80 90 a0 b0 c0 d0 e0 f0");
        let expected = decode("63cab7040953d051cd60e0e7ba70e18c");
        sub_bytes_enc(&mut state);
        assert_eq!(expected, state);
    }

    #[test]
    fn test_shift_rows_enc() {
        let mut state = decode("63cab7040953d051cd60e0e7ba70e18c");
        let expected = decode("6353e08c0960e104cd70b751bacad0e7");
        shift_rows_enc(&mut state);
        assert_eq!(expected, state);
    }

    #[test]
    fn test_mix_columns_enc() {
        let mut state = decode("6353e08c0960e104cd70b751bacad0e7");
        let expected = decode("5f72641557f5bc92f7be3b291db9f91a");
        mix_columns_enc(&mut state);
        assert_eq!(expected, state);
    }

    #[test]
    fn test_sub_bytes_dec() {
        let mut state = decode("7a9f102789d5f50b2beffd9f3dca4ea7");
        let expected = decode("bd6e7c3df2b5779e0b61216e8b10b689");

        sub_bytes_dec(&mut state);
        assert_eq!(expected, state);
    }

    #[test]
    fn test_shift_rows_dec() {
        let mut state = decode("7ad5fda789ef4e272bca100b3d9ff59f");
        let expected = decode("7a9f102789d5f50b2beffd9f3dca4ea7");
        shift_rows_dec(&mut state);
        assert_eq!(expected, state);
    }

    #[test]
    fn test_mix_columns_dec() {
        let mut state = decode("bd6e7c3df2b5779e0b61216e8b10b689");
        let expected = decode("4773b91ff72f354361cb018ea1e6cf2c");
        mix_columns_dec(&mut state);
        assert_eq!(expected, state);
    }
}
