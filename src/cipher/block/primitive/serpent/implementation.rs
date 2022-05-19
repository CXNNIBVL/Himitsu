use super::{
    standard::AlgorithmStandard,
    bitsliced::AlgorithmBitsliced
};
use crate::util::secure::Array;

pub type Serpent = SerpentImpl<AlgorithmStandard>;
pub type SerpentBitsliced = SerpentImpl<AlgorithmBitsliced>;

pub trait AlgorithmProvider {
    fn new() -> Self;
}

pub struct SerpentImpl<A: AlgorithmProvider> {
    algorithm: A,
    key: Array<u32, 132>
}

impl<A: AlgorithmProvider> SerpentImpl<A> {

    pub fn new(key: [u8; 32]) -> Self {
        let expanded_key = key_schedule(key);
        
        Self {
            algorithm: A::new(),
            key: Array::from(expanded_key)
        }
    }
}

fn key_schedule(key: [u8; 32]) -> [u32; 132] {

    // let user_key = transmute_key(key);
    [0;132]
}

fn transmute_key(key: [u8; 32]) -> [u32; 8] {

    let mut transmuted = [0; 8];

    for i in 0..8 {
        let start = i * 4;
        let end = start + 4;

        for ix in start..end {
            transmuted[i] <<= 8;
            transmuted[i] |= key[ix] as u32;
        }
    }

    transmuted
}

/*
    void w(uint32_t *w) {
        for (short i = 8; i < 140; i++) {
            w[i] = ROTL((w[i - 8] ^ w[i - 5] ^ w[i - 3] ^ w[i - 1] ^ FRAC ^ (i - 8)), 11);
        }
    }
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst() {
        let mut key = [0;32];
        for i in 0..32 {
            key[i] = i as u8;
        }

        let res = transmute_key(key);
        
        for word in res {
            for i in 0..4 {
                let sh = word >> i * 8;
                let byte = (sh & 0xFF) as u8;
                println!("{}", byte);
            }
        }
    }

    #[test]
    fn test_transmute_key() {
        
        let mut key = [0;32];
        for i in 0..32 {
            key[i] = i as u8;
        }

        let expected = [0; 8];
        

        let res = transmute_key(key);
        
        assert_eq!(res, expected);
    }
}