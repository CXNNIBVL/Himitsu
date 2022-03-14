use std::ops::BitXorAssign;

/// XORs src elements into dst
/// 
/// src and dst must be the same length, otherwise the function will panic
pub fn xor_buffers_unchecked<T: BitXorAssign + Copy>(dst: &mut [T], src: &[T]) {
    for i in 0..src.len() {
        dst[i] ^= src[i];
    }
}

/// XORs src elements into dst
/// 
/// XORs at least `min(src, dst)` elements
pub fn xor_buffers<T: BitXorAssign + Copy>(dst: &mut [T], src: &[T]) {
    use std::cmp::min;
    let s = min(src.len(), dst.len());
    for i in 0..s {
        dst[i] ^= src[i];
    }
}