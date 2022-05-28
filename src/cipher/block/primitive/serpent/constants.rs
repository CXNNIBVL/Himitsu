pub const FRAC: [u8; 4] = [0x9e, 0x37, 0x79, 0xb9];
pub const BLOCKSIZE: usize = 16;
pub const ROUNDS: usize = 32;
pub const SERPENT_128_KEYLEN: usize = 16;
pub const SERPENT_192_KEYLEN: usize = 24;
pub const SERPENT_256_KEYLEN: usize = 32;
pub const SERPENT_PADDED_KEYLEN: usize = SERPENT_256_KEYLEN;
pub const SERPENT_EXPANDED_KEYLEN: usize = 528;