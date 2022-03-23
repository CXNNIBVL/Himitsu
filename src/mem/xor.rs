use std::ops::BitXorAssign;

/// XORs src elements into dst and returns the number of xored elements
///
/// src and dst must be the same length, otherwise the function will panic
pub fn xor_buffers_unchecked<T: BitXorAssign + Clone>(dst: &mut [T], src: &[T]) -> usize {
    for i in 0..dst.len() {
        dst[i] ^= src[i].clone();
    }

    dst.len()
}

/// XORs src elements into dst and returns the number of xored elements
///
/// XORs at least `min(src, dst)` elements
pub fn xor_buffers<T: BitXorAssign + Clone>(dst: &mut [T], src: &[T]) -> usize {
    use std::cmp::min;
    let s = min(src.len(), dst.len());
    for i in 0..s {
        dst[i] ^= src[i].clone();
    }

    s
}
