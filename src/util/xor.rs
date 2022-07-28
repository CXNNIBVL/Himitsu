use std::ops::BitXorAssign;

/// XORs src elements into dst and returns the number of xored elements
///
/// XORs at least `min(src, dst)` elements
pub fn xor_buffers<T: BitXorAssign + Clone>(dst: &mut [T], src: &[T]) -> usize {

    let mut count = 0;
    dst.iter_mut().zip(src).for_each(|(d, s)| {
        *d ^= s.clone();
        count += 1;
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_buffers() {

        let mut a = [0, 1, 0, 1];
        let b = [0, 0, 1, 1];
        let eq = [0, 1, 1, 0];

        let count = xor_buffers(&mut a, &b);
        assert_eq!(count, 4);
        assert_eq!(a, eq);

    }
}
