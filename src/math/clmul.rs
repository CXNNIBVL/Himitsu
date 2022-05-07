pub fn carryless_multiply(mut x: u64, y: u64) -> u128 {
    let mut result = 0;

    if x == 0 || y == 0 {
        return result;
    }

    let mut pos = 0;
    while x != 0 {
        let z_trailing = x.trailing_zeros();
        x >>= z_trailing;
        pos += z_trailing;

        result ^= (y << pos) as u128;
        x >>= 1;
        pos += 1;
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn clmul1() {
        let x = 0b0110u64;
        let y = 0b1010u64;
        let e = 0b00111100u128;
        let r = carryless_multiply(x, y);
        assert_eq!(e, r)
    }

    #[test]
    fn clmul2() {
        let x = 0b0110u64;
        let y = 0b1010u64;
        let r1 = carryless_multiply(x, y);
        let r2 = carryless_multiply(y, x);
        assert_eq!(r1, r2)
    }

    // (x+y)*z = (x*z)+(y*z)
    #[test]
    fn clmul3() {
        let x = 0b0110u64;
        let y = 0b1010u64;
        let z = 0b1100u64;
        let r1 = carryless_multiply(x ^ y, z);
        let r2 = carryless_multiply(x, z) ^ carryless_multiply(y, z);
        assert_eq!(r1, r2)
    }

    // 0001000011111111111111111111110000000000111111100000000111111111
    // 0001000011111111111111111111110000000000111111100000000111111111
}
