mod implementation;
mod constants;

pub use implementation::{Serpent128, Serpent192, Serpent256};
pub use constants::{BLOCKSIZE, SERPENT_128_KEYLEN, SERPENT_192_KEYLEN, SERPENT_256_KEYLEN};