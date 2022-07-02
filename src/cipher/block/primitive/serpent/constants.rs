pub const FRAC: u32 = 0x9E3779B9;
pub const BLOCKSIZE: usize = 16;
pub const ROUNDS: usize = 32;

pub const SERPENT_128_KEYLEN: usize = 16;
pub const SERPENT_192_KEYLEN: usize = 24;
pub const SERPENT_256_KEYLEN: usize = 32;

pub const SERPENT_PADDED_KEYLEN: usize = SERPENT_256_KEYLEN;

/// Len in words
pub const W_SERPENT_EXPANDED_KEYLEN: usize = 140;

/// Keyspace index
pub const BEGIN_KEYSPACE: usize = 8;
pub const END_KEYSPACE: usize = W_SERPENT_EXPANDED_KEYLEN;

/*
    SBoxes are copied from Dag Arne Osvik's optimised SBoxes
*/

type SBoxFn = fn(&mut [u32]) -> ();

const S_BOXES: [SBoxFn; 8] = [
    sbox_0,	
    sbox_1,	
    sbox_2,	
    sbox_3,
	sbox_4,
    sbox_5,
    sbox_6,
    sbox_7
];

const INV_S_BOXES: [SBoxFn; 8] = [
    inv_sbox_0,
    inv_sbox_1,
    inv_sbox_2,
    inv_sbox_3,
    inv_sbox_4,
    inv_sbox_5,
    inv_sbox_6,
    inv_sbox_7
];

pub fn sbox(i: usize) -> SBoxFn {
	S_BOXES[i]
}

pub fn inv_sbox(i: usize) -> SBoxFn {
	INV_S_BOXES[i]
}

pub fn sbox_0(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r3 ^= r0; r4 = r1;
	r1 &= r3; r4 ^= r2;
	r1 ^= r0; r0 |= r3;
	r0 ^= r4; r4 ^= r3;
	r3 ^= r2; r2 |= r1;
	r2 ^= r4; r4 = !r4;
	r4 |= r1; r1 ^= r3;
	r1 ^= r4; r3 |= r0;
	r1 ^= r3; r4 ^= r3;

	words[0] = r1;
	words[1] = r4;
	words[2] = r2;
	words[3] = r0;
}

pub fn inv_sbox_0(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r2 = !r2; r4 = r1;
	r1 |= r0; r4 = !r4;
	r1 ^= r2; r2 |= r4;
	r1 ^= r3; r0 ^= r4;
	r2 ^= r0; r0 &= r3;
	r4 ^= r0; r0 |= r1;
	r0 ^= r2; r3 ^= r4;
	r2 ^= r1; r3 ^= r0;
	r3 ^= r1;
	r2 &= r3;
	r4 ^= r2;

	words[0] = r0;
	words[1] = r4;
	words[2] = r1;
	words[3] = r3;
}

pub fn sbox_1(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r0 = !r0; r2 = !r2;
	r4 = r0; r0 &= r1;
	r2 ^= r0; r0 |= r3;
	r3 ^= r2; r1 ^= r0;
	r0 ^= r4; r4 |= r1;
	r1 ^= r3; r2 |= r0;
	r2 &= r4; r0 ^= r1;
	r1 &= r2;
	r1 ^= r0; r0 &= r2;
	r0 ^= r4;

	words[0] = r2;
	words[1] = r0;
	words[2] = r3;
	words[3] = r1;
}

pub fn inv_sbox_1(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r4 = r1; r1 ^= r3;
	r3 &= r1; r4 ^= r2;
	r3 ^= r0; r0 |= r1;
	r2 ^= r3; r0 ^= r4;
	r0 |= r2; r1 ^= r3;
	r0 ^= r1; r1 |= r3;
	r1 ^= r0; r4 = !r4;
	r4 ^= r1; r1 |= r0;
	r1 ^= r0;
	r1 |= r4;
	r3 ^= r1;

	words[0] = r4;
	words[1] = r0;
	words[2] = r3;
	words[3] = r2;
}

pub fn sbox_2(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r4 = r0; r0 &= r2;
	r0 ^= r3; r2 ^= r1;
	r2 ^= r0; r3 |= r4;
	r3 ^= r1; r4 ^= r2;
	r1 = r3 ; r3 |= r4;
	r3 ^= r0; r0 &= r1;
	r4 ^= r0; r1 ^= r3;
	r1 ^= r4; r4 = !r4;

	words[0] = r2;
	words[1] = r3;
	words[2] = r1;
	words[3] = r4;
}

pub fn inv_sbox_2(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r2 ^= r3; r3 ^= r0;
	r4 = r3; r3 &= r2;
	r3 ^= r1; r1 |= r2;
	r1 ^= r4; r4 &= r3;
	r2 ^= r3; r4 &= r0;
	r4 ^= r2; r2 &= r1;
	r2 |= r0; r3 = !r3;
	r2 ^= r3; r0 ^= r3;
	r0 &= r1; r3 ^= r4;
	r3 ^= r0;

	words[0] = r1;
	words[1] = r4;
	words[2] = r2;
	words[3] = r3;
}

pub fn sbox_3(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r4 = r0;  r0 |= r3;
	r3 ^= r1; r1 &= r4;
	r4 ^= r2; r2 ^= r3;
	r3 &= r0; r4 |= r1;
	r3 ^= r4; r0 ^= r1;
	r4 &= r0; r1 ^= r3;
	r4 ^= r2; r1 |= r0;
	r1 ^= r2; r0 ^= r3;
	r2 = r1;  r1 |= r3;
	r1 ^= r0;

	words[0] = r1;
	words[1] = r2;
	words[2] = r3;
	words[3] = r4;
}

pub fn inv_sbox_3(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r4 = r2;  r2 ^= r1;
	r0 ^= r2; r4 &= r2;
	r4 ^= r0; r0 &= r1;
	r1 ^= r3; r3 |= r4;
	r2 ^= r3; r0 ^= r3;
	r1 ^= r4; r3 &= r2;
	r3 ^= r1; r1 ^= r0;
	r1 |= r2; r0 ^= r3;
	r1 ^= r4;
	r0 ^= r1;

	words[0] = r2;
	words[1] = r1;
	words[2] = r3;
	words[3] = r0;
}

pub fn sbox_4(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r1 ^= r3; r3 = !r3;
	r2 ^= r3; r3 ^= r0;
	r4 = r1;  r1 &= r3;
	r1 ^= r2; r4 ^= r3;
	r0 ^= r4; r2 &= r4;
	r2 ^= r0; r0 &= r1;
	r3 ^= r0; r4 |= r1;
	r4 ^= r0; r0 |= r3;
	r0 ^= r2; r2 &= r3;
	r0 = !r0; r4 ^= r2;

	words[0] = r1;
	words[1] = r4;
	words[2] = r0;
	words[3] = r3;
}

pub fn inv_sbox_4(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r4 = r2; r2 &= r3;
	r2 ^= r1; r1 |= r3;
	r1 &= r0; r4 ^= r2;
	r4 ^= r1; r1 &= r2;
	r0 = !r0; r3 ^= r4;
	r1 ^= r3; r3 &= r0;
	r3 ^= r2; r0 ^= r1;
	r2 &= r0; r3 ^= r0;
	r2 ^= r4;
	r2 |= r3; r3 ^= r0;
	r2 ^= r1;

	words[0] = r0;
	words[1] = r3;
	words[2] = r2;
	words[3] = r4;
}

pub fn sbox_5(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r0 ^= r1; r1 ^= r3;
	r3 = !r3; r4 = r1; 
	r1 &= r0; r2 ^= r3;
	r1 ^= r2; r2 |= r4;
	r4 ^= r3; r3 &= r1;
	r3 ^= r0; r4 ^= r1;
	r4 ^= r2; r2 ^= r0;
	r0 &= r3; r2 = !r2;
	r0 ^= r4; r4 |= r3;
	r2 ^= r4;

	words[0] = r1;
	words[1] = r3;
	words[2] = r0;
	words[3] = r2;
}

pub fn inv_sbox_5(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r1 = !r1; r4 = r3;
	r2 ^= r1; r3 |= r0;
	r3 ^= r2; r2 |= r1;
	r2 &= r0; r4 ^= r3;
	r2 ^= r4; r4 |= r0;
	r4 ^= r1; r1 &= r2;
	r1 ^= r3; r4 ^= r2;
	r3 &= r4; r4 ^= r1;
	r3 ^= r4; r4 = !r4;
	r3 ^= r0;

	words[0] = r1;
	words[1] = r4;
	words[2] = r3;
	words[3] = r2;
}

pub fn sbox_6(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r2 = !r2; r4 = r3; 
	r3 &= r0; r0 ^= r4;
	r3 ^= r2; r2 |= r4;
	r1 ^= r3; r2 ^= r0;
	r0 |= r1; r2 ^= r1;
	r4 ^= r0; r0 |= r3;
	r0 ^= r2; r4 ^= r3;
	r4 ^= r0; r3 = !r3;
	r2 &= r4;
	r2 ^= r3;

	words[0] = r0;
	words[1] = r1;
	words[2] = r4;
	words[3] = r2;
}

pub fn inv_sbox_6(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r0 ^= r2; r4 = r2;
	r2 &= r0; r4 ^= r3;
	r2 = !r2; r3 ^= r1;
	r2 ^= r3; r4 |= r0;
	r0 ^= r2; r3 ^= r4;
	r4 ^= r1; r1 &= r3;
	r1 ^= r0; r0 ^= r3;
	r0 |= r2; r3 ^= r1;
	r4 ^= r0;

	words[0] = r1;
	words[1] = r2;
	words[2] = r4;
	words[3] = r3;
}

pub fn sbox_7(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r4 = r1; r1 |= r2; 
	r1 ^= r3; r4 ^= r2;
	r2 ^= r1; r3 |= r4;
	r3 &= r0; r4 ^= r2;
	r3 ^= r1; r1 |= r4;
	r1 ^= r0; r0 |= r4;
	r0 ^= r2; r1 ^= r4;
	r2 ^= r1; r1 &= r0;
	r1 ^= r4; r2 = !r2;
	r2 |= r0;
	r4 ^= r2;

	words[0] = r4;
	words[1] = r3;
	words[2] = r1;
	words[3] = r0;
}

pub fn inv_sbox_7(words: &mut [u32]) {

	let mut r0 = words[0];
	let mut r1 = words[1];
	let mut r2 = words[2];
	let mut r3 = words[3];
	let mut r4;

	r4 = r2; r2 ^= r0;
	r0 &= r3; r4 |= r3;
	r2 = !r2; r3 ^= r1;
	r1 |= r0; r0 ^= r2;
	r2 &= r4; r3 &= r4;
	r1 ^= r2; r2 ^= r0;
	r0 |= r2; r4 ^= r1;
	r0 ^= r3; r3 ^= r4;
	r4 |= r0; r3 ^= r2;
	r4 ^= r2;

	words[0] = r3;
	words[1] = r0;
	words[2] = r1;
	words[3] = r4;
}
