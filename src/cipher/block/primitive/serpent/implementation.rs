use super::constants::*;
use crate::{util::secure::Array, array, mem};

pub type Serpent128 = Serpent<SERPENT_128_KEYLEN>;
pub type Serpent192 = Serpent<SERPENT_192_KEYLEN>;
pub type Serpent256 = Serpent<SERPENT_256_KEYLEN>;

pub struct Serpent<const INPUT_KEY_LEN: usize> {
    key: Array<u8, SERPENT_EXPANDED_KEYLEN>
}

impl<const IK: usize> Serpent<IK> {
    pub fn new(key: [u8; IK]) -> Self {
        let expanded_key = key_schedule( padding( array!(key) ) );
        
        Self {
            key: Array::from(expanded_key)
        }
    }


}

/// Applies the Serpent padding scheme to shorter keys
fn padding<const IK: usize>(key: Array<u8, IK>) -> Array<u8, SERPENT_PADDED_KEYLEN> {

    let mut padded = array![0; SERPENT_PADDED_KEYLEN];

    for i in 0..key.len() {
        padded[i] = key[i]
    }

    // Check for short key
    if key.len() < SERPENT_PADDED_KEYLEN {
        // Add single '1' bit
        padded[key.len()] = 0b10000000;
    }

    padded
}

fn key_schedule(key: Array<u8, SERPENT_PADDED_KEYLEN>) -> Array<u8, SERPENT_EXPANDED_KEYLEN> {

    let mut keys = array![0; SERPENT_EXPANDED_KEYLEN];
    
    for i in 0..key.len() {
        keys[i] = key[i];
    }

    let sections = [8,5,3,1];
    for i in 8..140 {

        let into_section_start = i * 4;
        let into_section_end = into_section_start + 4;

        // Assign Frac
        {
            let into = &mut keys[into_section_start..into_section_end];
            into.copy_from_slice(&FRAC);
        }

        // Xor Key sections into the new key section
        for s in sections.iter() {
            
            let (start, end) = ( (i - s) * 4, into_section_end);
            let section = &mut keys[start..end];

            keyschedule_xor_section(section); 
        }

        // Xor i into the key section
        let bytes = u32_as_le_bytes( (i - 8) as u32);
        keys[into_section_start] ^= bytes.0;
        keys[into_section_start + 1] ^= bytes.1;
        keys[into_section_start + 2] ^= bytes.2;
        keys[into_section_start + 3] ^= bytes.3;

        rotate_u32_from_raw_bytes(&mut keys[into_section_start..into_section_end], 11);
        
        // let tmp = prekeys[i - 8] ^ prekeys[i - 5] ^ prekeys[i - 3] ^ prekeys[i - 1] ^ FRAC ^ (i as u32 - 8);
        // prekeys[i] = tmp.rotate_left(11);
    }

    keys
}

fn rotate_u32_from_raw_bytes(bytes: &mut [u8], count: usize) {

}

fn u32_as_le_bytes(x: u32) -> (u8, u8, u8, u8) {
    let b1 = ( (x & 0xFF000000) >> 24) as u8;
    let b2 = ( (x & 0x00FF0000) >> 16) as u8;
    let b3 = ( (x & 0x0000FF00) >> 8) as u8;
    let b4 = (x & 0x000000FF) as u8;

    (b1, b2, b3, b4)
}

fn keyschedule_xor_section(section: &mut [u8]) {

    let (xor_section, rem) = section.split_at_mut(4);
    let into_ix = rem.len() - 4;

    mem::xor_buffers(&mut rem[into_ix..into_ix + 4], xor_section);
}

// fn key_schedule(key: Array<u8, SERPENT_PADDED_KEYLEN>) -> Array<u8, SERPENT_EXPANDED_KEYLEN> {

//     let mut keys = array![0; SERPENT_EXPANDED_KEYLEN];
    
//     for i in 0..key.len() {
//         keys[i] = key[i];
//     }

//     for i in 8..140 {

//         let into_ix = i * 4;

//         // Assign Frac
//         {
//             let into = &mut keys[into_ix..into_ix + 4];

//             into.copy_from_slice(&FRAC);
//         }

//         // Xor u32 key sections into the specific u32 into section
//         keyschedule_xor_sections(keys.as_mut(), i, 8);
//         keyschedule_xor_sections(keys.as_mut(), i, 5);
//         keyschedule_xor_sections(keys.as_mut(), i, 3);
//         keyschedule_xor_sections(keys.as_mut(), i, 1);

//         // Xor i into the key section
//         mem::xor_buffers(
//             &mut keys[into_ix..into_ix + 4], 
//             u32_as_array_u8( (i - 8) as u32).as_ref()
//         );
        
//         // let tmp = prekeys[i - 8] ^ prekeys[i - 5] ^ prekeys[i - 3] ^ prekeys[i - 1] ^ FRAC ^ (i as u32 - 8);
//         // prekeys[i] = tmp.rotate_left(11);
//     }

//     keys
// }

// fn u32_as_array_u8(x: u32) -> Array<u8, 4> {
//     let b1 = ( (x & 0xFF000000) >> 24) as u8;
//     let b2 = ( (x & 0x00FF0000) >> 16) as u8;
//     let b3 = ( (x & 0x0000FF00) >> 8) as u8;
//     let b4 = (x & 0x000000FF) as u8;

//     array![b1, b2, b3, b4]
// }

// fn keyschedule_xor_sections(keys: &mut [u8], i: usize, ix: usize) {
    
//     // Get section that starts at the position we want to xor
//     let split_ix = (i - ix) * 4;
//     let (_, section) = keys.split_at_mut(split_ix);

//     // Split off the first 4 bytes
//     let (xor_section, rem) = section.split_at_mut(4);

//     // Get the into section
//     let into_ix = ix * 4;
//     let into = &mut rem[into_ix..into_ix + 4];

//     mem::xor_buffers(into, xor_section);
// } 

#[cfg(test)]
mod tests {
    use super::*;

    
}