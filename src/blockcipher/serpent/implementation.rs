
use crate::{util, util::Array, array};
use crate::blockcipher::{
    BlockCipherEncryption, 
    BlockCipherDecryption
};
use super::constants::*;

pub type Serpent128 = Serpent<SERPENT_128_KEYLEN>;
pub type Serpent192 = Serpent<SERPENT_192_KEYLEN>;
pub type Serpent256 = Serpent<SERPENT_256_KEYLEN>;

type ExpandedKey = Array<u32, W_SERPENT_EXPANDED_KEYLEN>;

pub struct Serpent<const INPUT_KEY_LEN: usize> {
    key: ExpandedKey
}

impl<const IK: usize> Serpent<IK> {
    pub fn new(key: [u8; IK]) -> Self {
        Self {
            key: expanded_key(array!(key))
        }
    }


}

fn expanded_key<const IK: usize>(input_key: Array<u8, IK>) -> ExpandedKey {
    use std::iter::{repeat, once, zip};

    // Apply Padding [00..01]KEY_MATERIAL .
    // Reverse sequence and take up to
    // the padded keylength
    let padded_iter = repeat(0)
    .chain(once(1))
    .chain(input_key.into_iter())
    .rev().take(SERPENT_PADDED_KEYLEN);

    let bytes_w_shifts = zip(
        padded_iter,
        [0,8,16,24].into_iter().cycle()
    );

    let mut start_key = array![0; W_SERPENT_EXPANDED_KEYLEN];

    // Deserialize into u32 array
    for (ix, (byte, shift)) in bytes_w_shifts.enumerate() {
        start_key[ix / 4] |= (byte as u32) << shift;
    }

    apply_key_schedule(start_key)
}

fn apply_key_schedule(mut key: ExpandedKey) -> ExpandedKey {

    // Compute pre-keys
    for ix in BEGIN_KEYSPACE..END_KEYSPACE {
        let word = key[ix - 8] ^ key[ix - 5] ^ key[ix - 3] ^ key[ix - 1] ^ FRAC ^ (ix as u32 - 8);
        key[ix] = word.rotate_left(11);
    }

    // Iterator starting at 3, counting downwards, cycling continuously.
    // Get corresponding SBox
    let sbox_iter = (0..=7).rev()
    .cycle()
    .skip(4)
    .map(|bx|{ sbox(bx) });

    let key_space = &mut key[BEGIN_KEYSPACE..END_KEYSPACE];
    let block_w_sbox = key_space.chunks_mut(4).zip(sbox_iter);

    for (block, sbox) in block_w_sbox {
        
        // Apply SBox and place in LE 128bit round key
        (block[3], block[2], block[1], block[0]) = sbox(block[0], block[1], block[2], block[3]);
    }

    key

}

fn linear_transformation(mut x0: u32, mut x1: u32, mut x2: u32, mut x3: u32) -> (u32, u32, u32, u32) {

    x0 = x0.rotate_left(13);
    x2 = x2.rotate_left(3);
    x1 ^= x0 ^ x2;
    x3 ^= x2 ^ (x0 << 3);
    x1 = x1.rotate_left(1);
    x3 = x3.rotate_left(7);
    x0 ^= x1 ^ x3;
    x2 ^= x3 ^ (x1 << 7);
    x0 = x0.rotate_left(5);
    x2 = x2.rotate_left(22);

    (x0, x1, x2, x3)
}

fn inv_linear_transformation(x: &mut [u32]) {
    x[2] = x[2].rotate_right(22);    
    x[0] = x[0].rotate_right(5);     
    x[2] ^= x[3] ^ (x[1] << 7);  
    x[0] ^= x[1] ^ x[3];         
    x[1] = x[1].rotate_right(1);     
    x[3] = x[3].rotate_right(7) ^ x[2] ^ (x[0] << 3);  
    x[1] ^= x[0] ^ x[2];         
    x[2] = x[2].rotate_right(3);     
    x[0] = x[0].rotate_right(13);
}

fn mix_key(x: &mut [u32], k: &[u32]) {
    util::xor_buffers(x, k);
}

fn roundkey(round: usize, keys: &[u32]) -> &[u32] {
    let keyspace = &keys[BEGIN_KEYSPACE..END_KEYSPACE];
    
    let roundkey_begin = round * 4;
    let roundkey_end = roundkey_begin + 4;

    &keyspace[roundkey_begin..roundkey_end]
}

fn data_to_block(data: &[u8]) -> Array<u32, 4> {
    
    let mut block = array![0; 4];

    for (i, by) in data.iter().rev().enumerate() {
        block[i / 4] |= (*by as u32) << (24 - (i % 4) * 8);
    }

    block
}

fn block_to_data(block: &[u32], data: &mut [u8]) {

    let it = data.chunks_mut(4).zip(block.iter());

    for (chunk, word) in it {

        chunk[3] = (*word & 0x000000FF) as u8;
        chunk[2] = ((*word & 0x0000FF00) >> 8) as u8;
        chunk[1] = ((*word & 0x00FF0000) >> 16) as u8;
        chunk[0] = ((*word & 0xFF000000) >> 24) as u8;
    }

}

fn log_block(label: String, block: &[u32]) {
    let f = format!("{}{:08x?}",label, block.as_ref());
    dbg!(f);
}

impl<const IK: usize> BlockCipherEncryption<BLOCKSIZE> for Serpent<IK> {

    fn encrypt(&mut self, data: &mut [u8; BLOCKSIZE]) {

        let mut block = data_to_block(data);
        
        for round in 0..ROUNDS {
            
            mix_key(block.as_mut(), roundkey(round, self.key.as_ref()));
            
            (block[0], block[1], block[2], block[3]) = sbox(round)(block[3], block[2], block[1], block[0]);

            (block[3], block[2], block[1], block[0]) = linear_transformation(block[0], block[1], block[2], block[3]);
        }

        const LAST_ROUND: usize = ROUNDS;

        mix_key(block.as_mut(), roundkey(LAST_ROUND, self.key.as_ref()));
        
        (block[3], block[2], block[1], block[0]) = sbox(LAST_ROUND)(block[3], block[2], block[1], block[0]);

        mix_key(block.as_mut(), roundkey(LAST_ROUND + 1, self.key.as_ref()));

        block_to_data(block.as_ref(), data);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex_string(hex: &str) -> Vec<u8> {
        use crate::encode::hex::HexDecoder;
        hex.chars().decode_hex()
    }

    fn deserialize_u32_into_u8(words: &[u32]) -> Vec<u8> {

        let mut vec = Vec::new();

        for word in words {
            let b1 = ( (*word & 0xFF000000) >> 24) as u8;
            let b2 = ( (*word & 0x00FF0000) >> 16) as u8;
            let b3 = ( (*word & 0x0000FF00) >> 8) as u8;
            let b4 = (*word & 0x000000FF) as u8;

            vec.push(b1);
            vec.push(b2);
            vec.push(b3);
            vec.push(b4);
        }

        vec
    }

    #[test]
    fn test_serpent() {

        // let input_key = [
        //     0x00,
        //     0x01,
        //     0x02,
        //     0x03,
        //     0x04,
        //     0x05,
        //     0x06,
        //     0x07,
        //     0x08,
        //     0x09,
        //     0xaa,
        //     0xbb,
        //     0xcc,
        //     0xdd,
        //     0xee,
        //     0xff
        // ];

        let mut input_key = [0; 32]; input_key[31] = 0x80;

        let mut serpent = Serpent256::new(input_key);

        let mut data = [0; 16];
        serpent.encrypt(&mut data);

        println!("{:02x?}", data);
    }

    #[test]
    fn test_expandable_key_input16() {

        let input_key = array![0xaa; 16];

        let expected = [
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0x00000001,
            0x00000000,
            0x00000000,
            0x00000000
        ];

        let res_words = expanded_key(input_key);
        
        assert!(
            res_words.into_iter().take(8)
            .eq(expected.into_iter())
        )
    }

    #[test]
    fn test_expandable_key_input24() {

        let input_key = array![0xaa; 24];

        let expected = [
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0x00000001,
            0x00000000
        ];

        let res_words = expanded_key(input_key);
        
        assert!(
            res_words.into_iter().take(8)
            .eq(expected.into_iter())
        )

    }

    #[test]
    fn test_expandable_key_input32() {

        let input_key = array![0xaa; 32];

        let expected = [
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa,
            0xaaaaaaaa
        ];

        let res_words = expanded_key(input_key);
        
        assert!(
            res_words.into_iter().take(8)
            .eq(expected.into_iter())
        )

    }

    #[test]
    fn test_key_schedule() {

        let input_key = array![
            0x00,
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
            0x09,
            0xaa,
            0xbb,
            0xcc,
            0xdd,
            0xee,
            0xff
        ];

        macro_rules! expected_hex {
            () => {
                "41e1e73bb2a748c837ea85e4dd6e8487
                0e3541ed888a7bd0850cdece6778fa19
                7899f56ec717e6f2f673a40031a922bc
                944da91f532fb8c0782b4abac5adda7f
                3df19f257eb22f58e9f7abcf83f6f334
                fd7b224be2dd5bb3fdf8dfb95e59b7dd
                adef3f999cd1c64637ca7ffc5da88d7a
                8604c19060bde15ab8a39b117e6528f5
                0a071549f00ddf4a4846794e4bc381ce
                8b94abb33674ec53cceaa0330f024302
                74c012955ea309db42c0280447e067e0
                62c14e306a31498ee93eb992ad84cb7b
                c5e668099b0745dec9217167e03eb45f
                7fbc7710169b12787333ee79baa5e0e0
                f388a3c7c7c3c874ae8ced2728a6dd51
                47ac46700af4f01c2d922e5d058acf6a
                ee8e99c59760af7ac4488e2ec6523911
                16f6785066df5f571ef05de6ea38f5d6
                eebdc74ce1b71ed0bd5c6355731a0df6
                81bfeee03122e674dc0a3a5f67f5833a
                0765a42bc2102db50a8e4dd273b0c29b
                f1a8b3f0778ed401d2495d437f953c35
                34382bc44e53ce391e0e45a477313996
                30672ce6f10c668b67a8db44b182762e
                8d013694b5fe369a45de0d179f3a3ecf
                5a4aba9940c939edc373445515382677
                d4914fbff4e972521cab19ba3cf7d56d
                e74ba1f7ef2bc58e436b3d8f707db88a
                b9dd1a7c89dc175ab99813ea4d254430
                c279d19fcf5b78a918c2294173e4d2f2
                6aedd443659ee5c6a0f6b78413739bf9
                0e6b2b9fd64c9489d0819ffa2bd0c648
                73ca9671b185a3a09a9aace47a77ea3a
                "
            };
        }

        let expected = decode_hex_string(expected_hex!());

        let res = apply_key_schedule(
            expanded_key(input_key)
        );

        let res_bytes = deserialize_u32_into_u8(&res[BEGIN_KEYSPACE..END_KEYSPACE]);

        assert_eq!(res_bytes, expected);
    }

}