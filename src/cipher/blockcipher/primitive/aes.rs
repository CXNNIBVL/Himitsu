pub use crate::traits::block_primitive::{
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption, 
    BlockCipherPrimitiveInfo as PrimitiveInfo
};

pub use crate::util::buffer::FixedBuffer;

const AES_BLOCKSIZE: usize = 16;
const AES_KEYLEN_MIN: usize = 16;
const AES_KEYLEN_MAX: usize = 32;
type AesBlockType = FixedBuffer<u8, AES_BLOCKSIZE>;

/// Aes Encryption and Decryption provider
pub struct Aes { config: AesConfig }

impl PrimitiveInfo for Aes {
    const BLOCKSIZE: usize = AES_BLOCKSIZE;
    const KEYLEN_MIN: usize = AES_KEYLEN_MIN;
    const KEYLEN_MAX: usize = AES_KEYLEN_MAX;
    type BlockType = AesBlockType;
}

impl PrimitiveEncryption for Aes {

    fn new(key: &[u8]) -> Self {
        Self { config: aes_configuration(key) }
    }

    fn mutate(&self, mut_block: &mut Self::BlockType, xor_block: Option<&Self::BlockType>) {

    }
}

impl PrimitiveDecryption for Aes {

    fn new(key: &[u8]) -> Self {
        Self { config: aes_configuration(key) }
    }

    fn mutate(&self, mut_block: &mut Self::BlockType, xor_block: Option<&Self::BlockType>) {

    }
}

struct AesConfig {
    expanded_key: Vec<u8>,
    rounds: usize
}

fn aes_configuration(key: &[u8]) -> AesConfig {
    AesConfig { expanded_key: Vec::new(), rounds: 12 }
}

