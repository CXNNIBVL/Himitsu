use std::cmp::min;

/// XORs at least std:cmp::min(dst.len(), src.len()) src bytes into dst
pub fn xor_buffers(dst: &mut [u8], src: &[u8]) {
    let len = min(dst.len(), src.len());

    for i in 0..len {
        dst[i] ^= src[i];
    }
}