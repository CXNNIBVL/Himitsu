use std::ops::{Rem, Mul, Sub, Div};

pub trait ExtendedGcd: Copy + PartialEq<Self> + Div<Output=Self> + Rem<Output=Self> + Sub<Output=Self> + Mul<Output=Self> {

    const ZERO: Self;
    const ONE: Self;

    // Returns (gcd, First Bezout's coefficient, second Bezout's Coefficient
    fn extended_gcd(mut x: Self, mut y: Self) -> (Self,Self,Self) {
        let (mut a0,mut a1,mut b0,mut b1) = (Self::ONE, Self::ZERO, Self::ZERO, Self::ONE);

        while y != Self::ZERO {
            let (q,r)  = (x / y, x % y);
            let (c, d) = ( a0 - q * a1, b0 - q * b1  ); 

            x = y;
            y = r;
            a0 = a1;
            a1 = c;
            b0 = b1; 
            b1 = d;

        }

        (x, a0, b0)
    }
}

impl ExtendedGcd for u8 { const ZERO: Self = 0; const ONE: Self = 1; }
impl ExtendedGcd for u16 { const ZERO: Self = 0; const ONE: Self = 1; }
impl ExtendedGcd for u32 { const ZERO: Self = 0; const ONE: Self = 1; }
impl ExtendedGcd for u64 { const ZERO: Self = 0; const ONE: Self = 1; }
impl ExtendedGcd for usize { const ZERO: Self = 0; const ONE: Self = 1; }

impl ExtendedGcd for i8 { const ZERO: Self = 0; const ONE: Self = 1; }
impl ExtendedGcd for i16 { const ZERO: Self = 0; const ONE: Self = 1; }
impl ExtendedGcd for i32 { const ZERO: Self = 0; const ONE: Self = 1; }
impl ExtendedGcd for i64 { const ZERO: Self = 0; const ONE: Self = 1; }
impl ExtendedGcd for isize { const ZERO: Self = 0; const ONE: Self = 1; }

mod gf {

    pub fn multiplicative_inverse(sel: u8) -> u8 {
        let mut p = 0;

        for x in 0u8..=255u8 {
            // If zero, the multiplication is results in GF(1)
            // If non-zero, the multiplication ends with something different.
            let y = mul(sel,x) ^ 1;

            // OR all bits together in the rightmost bit. If y is zero, that means that the
            // result of ORing all bits together will also be zero. Otherwise, it will be 1.
            let or = y | y >> 1 | y >> 2 | y >> 3 | y >> 4 | y >> 5 | y >> 6 | y >> 7;

            // Extend the bits to the full byte and negate it. This means that the AND will
            // be zero if the multiplication in y was 1.
            p ^= !extend_bit(or) & x;
        }

        p
    }
    fn extend_bit(input: u8) -> u8 {
    (((input) as i8) << 7).wrapping_shr(7) as u8
    }

    pub fn mul(sel: u8, rhs: u8) -> u8 {
        let mut a = sel;
        let mut b = rhs;

        let mut p = 0;

        // Implementation details from https://en.wikipedia.org/wiki/Finite_field_arithmetic
        // Run the following loop eight times (once per bit).
        for _ in 0..8 {
            // If the rightmost bit of b is set, exclusive OR the product p by the value of a.
            // This is polynomial addition.
            p ^= extend_bit(b & 1) & a;

            // Shift b one bit to the right, discarding the rightmost bit, and making the leftmost
            // bit have a value of zero. This divides the polynomial by x, discarding the x0 term.
            b >>= 1;

            // Keep track of whether the leftmost bit of a is set to one and call this value carry.
            let carry = (a >> 7) & 1;

            // Shift a one bit to the left, discarding the leftmost bit, and making the new
            // rightmost bit zero. This multiplies the polynomial by x, but we still need to take
            // account of carry which represented the coefficient of x7.
            a <<= 1;

            // If carry had a value of one, exclusive or a with the hexadecimal
            // number 0x1b (00011011 in binary). 0x1b corresponds to the irreducible polynomial with
            // the high term eliminated. Conceptually, the high term of the irreducible polynomial
            // and carry add modulo 2 to 0.
            a ^= extend_bit(carry & 1) & 0x1b;
        }

       p
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn testing() {
        let poly = 0x1B;
        let a = 0x1B;
        let b = 148;
        let (gcd, bez1, bez2) = i64::extended_gcd(b.clone(), a.clone());
        println!("{:?} {:?} {:?}", gcd, bez1, bez2);
        println!("{}", gf::mul(b as u8, gcd as u8));
    }

    #[test]
    fn testing_1() {
        let el = 148;
        let inv = gf::multiplicative_inverse(el);
        println!("inv = {}", inv);
        let m = gf::mul(el, inv);
        println!("m = {}", m);
    }

}
