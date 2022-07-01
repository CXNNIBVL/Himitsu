use super::constants::*;
use crate::{util::secure::Array, array, mem};

pub type Serpent128 = Serpent<SERPENT_128_KEYLEN>;
pub type Serpent192 = Serpent<SERPENT_192_KEYLEN>;
pub type Serpent256 = Serpent<SERPENT_256_KEYLEN>;

pub struct Serpent<const INPUT_KEY_LEN: usize> {
    key: Array<u32, SERPENT_EXPANDED_KEYLEN>
}

impl<const IK: usize> Serpent<IK> {
    pub fn new(key: [u8; IK]) -> Self {
        Self {
            key: key_schedule( array!(key) )
        }
    }


}

fn generate_start_key_from_input_key<const IK: usize>(input_key: Array<u8, IK>) -> Array<u32, SERPENT_EXPANDED_KEYLEN> {

    let mut start_key = array![0; SERPENT_EXPANDED_KEYLEN];

    // Iterate over bytes, apply serpent padding (0x80) if necessary,
    // take only up to the padded length
    let iter = input_key.into_iter()
                                    .chain([0x80].into_iter())
                                    .take(SERPENT_PADDED_KEYLEN);

    serialize_bytes_iter_into_u32_slice(start_key.as_mut(), iter);

    start_key
}

fn serialize_bytes_iter_into_u32_slice<'a, I>(into: &mut [u32], iter: I) 
    where I: Iterator<Item = u8>
{
    for (i, byte) in iter.enumerate() {
        into[i / 4] |= (byte as u32) << (24 - (i % 4) * 8 );
    }
}
 
fn key_schedule<const IK: usize>(input_key: Array<u8, IK>) -> Array<u32, SERPENT_EXPANDED_KEYLEN> {

    let mut keys = generate_start_key_from_input_key(input_key);

    // Compute pre-keys
    for ix in 8..SERPENT_EXPANDED_KEYLEN {
        let word = keys[ix - 8] ^ keys[ix - 5] ^ keys[ix - 3] ^ keys[ix - 1] ^ FRAC ^ (ix as u32 - 8);
        keys[ix] = word.rotate_left(11);
    }

    // Iterator starting at 3, counting downwards, cycling continuously.
    // Get corresponding SBox
    let sbox_iter = (0..=7).rev()
    .cycle()
    .skip(4)
    .map(|sbox|{ S_BOXES[sbox] });

    let block_w_sbox = keys.chunks_mut(4).zip(sbox_iter);

    // Apply SBox
    for (block, sbox) in block_w_sbox {
        sbox(block);
    }

    keys
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
            let b4 = ( (*word & 0x000000FF) >> 0) as u8;

            vec.push(b1);
            vec.push(b2);
            vec.push(b3);
            vec.push(b4);
        }

        vec
    }

    #[test]
    fn test_generate_start_subkey_input0() {
        use std::iter;

        let input_key = array![0u8;0];

        // 0x8000..00, len = SERPENT_PADDED_KEYLEN
        let expected_bytes: Vec<u8> = iter::repeat(0x80u8)
                                .take(1)
                                .chain(iter::repeat(0))
                                .take(SERPENT_PADDED_KEYLEN).collect();

        let res_words = generate_start_key_from_input_key(input_key);
        let res_bytes = deserialize_u32_into_u8(res_words.as_ref());

        assert!(
            res_bytes.into_iter().take(SERPENT_PADDED_KEYLEN)
            .eq(expected_bytes.into_iter())
        );
    }

    #[test]
    fn test_generate_start_subkey_input16() {
        use std::iter;

        let input_key = array![0xAA; 16];

        // 0xAA..AA8000..00; 
        // 16 x AA, 1 x 80, 15 x 0; 
        // len = SERPENT_PADDED_KEYLEN
        let expected_bytes: Vec<u8> = iter::repeat(0xAAu8).take(16)
                                        .chain(iter::repeat(0x80u8).take(1))
                                        .chain(iter::repeat(0u8))
                                        .take(SERPENT_PADDED_KEYLEN)
                                        .collect();
                                        

        let res_words = generate_start_key_from_input_key(input_key);
        let res_bytes = deserialize_u32_into_u8(res_words.as_ref());

        assert!(
            res_bytes.into_iter().take(SERPENT_PADDED_KEYLEN)
            .eq(expected_bytes.into_iter())
        );
    }

    #[test]
    fn test_generate_start_subkey_input_full() {
        use std::iter;

        let input_key = array![0xAA; SERPENT_PADDED_KEYLEN];

        // 32 x 0xAA; 
        // len = SERPENT_PADDED_KEYLEN
        let expected_bytes: Vec<u8> = iter::repeat(0xAAu8)
                                        .take(SERPENT_PADDED_KEYLEN)
                                        .collect();
                                        

        let res_words = generate_start_key_from_input_key(input_key);
        let res_bytes = deserialize_u32_into_u8(res_words.as_ref());

        assert!(
            res_bytes.into_iter().take(SERPENT_PADDED_KEYLEN)
            .eq(expected_bytes.into_iter())
        );
    }

    #[test]
    fn test_generate_start_subkey_input_bigger_than_full() {
        use std::iter;

        let input_key = array![0xAA; SERPENT_PADDED_KEYLEN + 1];

        // 32 x 0xAA 
        // len = SERPENT_PADDED_KEYLEN
        let expected_bytes: Vec<u8> = iter::repeat(0xAAu8)
                                        .take(SERPENT_PADDED_KEYLEN)
                                        .collect();
                                        

        let res_words = generate_start_key_from_input_key(input_key);
        let res_bytes = deserialize_u32_into_u8(res_words.as_ref());

        assert!(
            res_bytes.into_iter().take(SERPENT_PADDED_KEYLEN)
            .eq(expected_bytes.into_iter())
        );
    }

    macro_rules! expected_key_0 {
        () => {
            "
            80000000000000000000000000000000
            40000000000000000000000000000000
            20000000000000000000000000000000
            10000000000000000000000000000000
            08000000000000000000000000000000
            04000000000000000000000000000000
            02000000000000000000000000000000
            01000000000000000000000000000000
            00800000000000000000000000000000
            00400000000000000000000000000000
            00200000000000000000000000000000
            00100000000000000000000000000000
            00080000000000000000000000000000
            00040000000000000000000000000000
            00020000000000000000000000000000
            00010000000000000000000000000000
            00008000000000000000000000000000
            00004000000000000000000000000000
            00002000000000000000000000000000
            00001000000000000000000000000000
            00000800000000000000000000000000
            00000400000000000000000000000000
            00000200000000000000000000000000
            00000100000000000000000000000000
            00000080000000000000000000000000
            00000040000000000000000000000000
            00000020000000000000000000000000
            00000010000000000000000000000000
            00000008000000000000000000000000
            00000004000000000000000000000000
            00000002000000000000000000000000
            00000001000000000000000000000000
            00000000800000000000000000000000
            00000000400000000000000000000000
            00000000200000000000000000000000
            00000000100000000000000000000000
            00000000080000000000000000000000
            00000000040000000000000000000000
            00000000020000000000000000000000
            00000000010000000000000000000000
            00000000008000000000000000000000
            00000000004000000000000000000000
            00000000002000000000000000000000
            00000000001000000000000000000000
            00000000000800000000000000000000
            00000000000400000000000000000000
            00000000000200000000000000000000
            00000000000100000000000000000000
            00000000000080000000000000000000
            00000000000040000000000000000000
            00000000000020000000000000000000
            00000000000010000000000000000000
            00000000000008000000000000000000
            00000000000004000000000000000000
            00000000000002000000000000000000
            00000000000001000000000000000000
            00000000000000800000000000000000
            00000000000000400000000000000000
            00000000000000200000000000000000
            00000000000000100000000000000000
            00000000000000080000000000000000
            00000000000000040000000000000000
            00000000000000020000000000000000
            00000000000000010000000000000000
            00000000000000008000000000000000
            00000000000000004000000000000000
            00000000000000002000000000000000
            00000000000000001000000000000000
            00000000000000000800000000000000
            00000000000000000400000000000000
            00000000000000000200000000000000
            00000000000000000100000000000000
            00000000000000000080000000000000
            00000000000000000040000000000000
            00000000000000000020000000000000
            00000000000000000010000000000000
            00000000000000000008000000000000
            00000000000000000004000000000000
            00000000000000000002000000000000
            00000000000000000001000000000000
            00000000000000000000800000000000
            00000000000000000000400000000000
            00000000000000000000200000000000
            00000000000000000000100000000000
            00000000000000000000080000000000
            00000000000000000000040000000000
            00000000000000000000020000000000
            00000000000000000000010000000000
            00000000000000000000008000000000
            00000000000000000000004000000000
            00000000000000000000002000000000
            00000000000000000000001000000000
            00000000000000000000000800000000
            00000000000000000000000400000000
            00000000000000000000000200000000
            00000000000000000000000100000000
            00000000000000000000000080000000
            00000000000000000000000040000000
            00000000000000000000000020000000
            00000000000000000000000010000000
            00000000000000000000000008000000
            00000000000000000000000004000000
            00000000000000000000000002000000
            00000000000000000000000001000000
            00000000000000000000000000800000
            00000000000000000000000000400000
            00000000000000000000000000200000
            00000000000000000000000000100000
            00000000000000000000000000080000
            00000000000000000000000000040000
            00000000000000000000000000020000
            00000000000000000000000000010000
            00000000000000000000000000008000
            00000000000000000000000000004000
            00000000000000000000000000002000
            00000000000000000000000000001000
            00000000000000000000000000000800
            00000000000000000000000000000400
            00000000000000000000000000000200
            00000000000000000000000000000100
            00000000000000000000000000000080
            00000000000000000000000000000040
            00000000000000000000000000000020
            00000000000000000000000000000010
            00000000000000000000000000000008
            00000000000000000000000000000004
            00000000000000000000000000000002
            00000000000000000000000000000001"
        };
    }
    
    #[test]
    fn test_key_schedule() {

        let input_key = array![0u8; 0];

        let expected_key_hex = expected_key_0!();

        let expected_bytes = decode_hex_string(expected_key_hex);
        
        let res_words = key_schedule(input_key);
        let res_bytes = deserialize_u32_into_u8(res_words.as_ref());

        // assert_eq!(res_bytes, expected_bytes);
        println!("########\nRES\n#######{:02X?}", res_bytes);
        println!("########\nEXP\n#######{:02X?}", expected_bytes);

    }

    #[test]
    fn test_key_schedule2() {

        // let input_key = array![
        //     0x00, 0x00, 0x00, 0x00, 
        //     0x00, 0x00, 0x00, 0x00, 
        //     0x00, 0x00, 0x00, 0x00, 
        //     0x00, 0x00, 0x00, 0x01,
        //     0x00, 0x11, 0x22, 0x33, 
        //     0x44, 0x55, 0x66, 0x77, 
        //     0x88, 0x99, 0xaa, 0xbb, 
        //     0xcc, 0xdd, 0xee, 0xff
        // ];

        let input_key = array![
            0xFF,0xEE,0xDD,0xCC,
            0xBB,0xAA,0x99,0x88,
            0x77,0x66,0x55,0x44,
            0x33,0x22,0x11,0x00,
            0x80,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00
        ];

        // for by in input_key.iter().rev() {
        //     print!("0x{:02X},", by);
        // }

        let expected_key_hex = expected_key_0!();

        let expected_bytes = decode_hex_string(expected_key_hex);
        
        let res_words = key_schedule(input_key);
        let res_bytes = deserialize_u32_into_u8(res_words.as_ref());

        // assert_eq!(res_bytes, expected_bytes);
        println!("########\nRES\n#######");
        println!("{:02X?}", res_bytes);
        // println!("########\nEXP\n#######{:02X?}", expected_bytes);

    }

}