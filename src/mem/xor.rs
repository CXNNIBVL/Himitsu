use std::ops::BitXorAssign;

/// XORs src bytes into dst
/// 
/// src and dst must be the same length, otherwise the function will panic
pub fn xor_buffers<T: BitXorAssign + Copy>(dst: &mut [T], src: &[T]) {
    for i in 0..src.len() {
        dst[i] ^= src[i];
    }
}