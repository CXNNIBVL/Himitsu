pub const FRAC: u32 = 0x9E3779B9;
pub const BLOCKSIZE: usize = 16;
pub const ROUNDS: usize = 32;
pub const SERPENT_128_KEYLEN: usize = 16;
pub const SERPENT_192_KEYLEN: usize = 24;
pub const SERPENT_256_KEYLEN: usize = 32;
pub const SERPENT_PADDED_KEYLEN: usize = SERPENT_256_KEYLEN;
pub const SERPENT_EXPANDED_KEYLEN: usize = 132; // u32

type WordsAsWord<'a> = (&'a mut u32, &'a mut u32, &'a mut u32, &'a mut u32);
fn words_as_word_mut<'a>(words: &'a mut [u32]) -> WordsAsWord<'a> {
    let (a, rema) = words.split_first_mut().unwrap();
    let (b, remb) = rema.split_first_mut().unwrap();
    let (c, remc) = remb.split_first_mut().unwrap();
    let (d, _) = remc.split_first_mut().unwrap();

    (a,b,c,d)
}

/*
    SBoxes are copied from Serpents reference implementation 
    (the optimized one, contained in `floppy2' of the AES Submission Package)
*/

const S_BOXES: [fn(&mut [u32]) -> (); 8] = [
    sbox_0,
    sbox_1,
    sbox_2,
    sbox_3,
    sbox_4,
    sbox_5,
    sbox_6,
    sbox_7
];

const INV_S_BOXES: [fn(&mut [u32]) -> (); 8] = [
    inv_sbox_0,
    inv_sbox_1,
    inv_sbox_2,
    inv_sbox_3,
    inv_sbox_4,
    inv_sbox_5,
    inv_sbox_6,
    inv_sbox_7
];

pub fn sbox(sbox_index: usize, words: &mut [u32]) {
    S_BOXES[sbox_index](words);
}

pub fn inv_sbox(sbox_index: usize, words: &mut [u32]) {
    INV_S_BOXES[sbox_index](words)
}


#[allow(non_snake_case, unused_assignments)]
pub fn sbox_0(words: &mut [u32]) {

    let mut t02 = 0; 
    let mut t03 = 0; 
    let mut t05 = 0; 
    let mut t06 = 0; 
    let mut t07 = 0; 
    let mut t08 = 0; 
    let mut t09 = 0; 
    let mut t11 = 0; 
    let mut t12 = 0; 
    let mut t13 = 0; 
    let mut t14 = 0; 
    let mut t15 = 0; 
    let mut t17 = 0; 
    let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

    t01 = *b ^ *c;
	t02 = *a | *d;
	t03 = *a ^ *b;
	*d = t02 ^ t01;
	t05 = *c | *d;
	t06 = *a ^ *d;
	t07 = *b | *c;
	t08 = *d & t05;
	t09 = t03 & t07;
	*c = t09 ^ t08;
    t11 = t09 & *c;
	t12 = *c ^ *d  ;
	t13 = t07 ^ t11;
	t14 = *b & t06;
	t15 = t06 ^ t13;
	*a = !t15;
	t17 = *a ^ t14;
	*b = t12 ^ t17;

}

#[allow(non_snake_case, unused_assignments)]
pub fn inv_sbox_0(words: &mut [u32]) {

    let mut t02 = 0; 
    let mut t03 = 0; 
    let mut t04 = 0; 
    let mut t05 = 0; 
    let mut t06 = 0; 
    let mut t08 = 0; 
    let mut t09 = 0; 
    let mut t10 = 0; 
    let mut t12 = 0; 
    let mut t13 = 0; 
    let mut t14 = 0; 
    let mut t15 = 0;
    let mut t17 = 0; 
    let mut t18 = 0; 
    let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *c   ^ *d  ;
	t02 = *a   | *b  ;
	t03 = *b   | *c  ;
	t04 = *c   & t01;
	t05 = t02 ^ t01;
	t06 = *a   | t04;
	*c   =     !t05;
	t08 = *b   ^ *d  ;
	t09 = t03 & t08;
	t10 = *d   | *c  ;
	*b   = t09 ^ t06;
	t12 = *a   | t05;
	t13 = *b   ^ t12;
	t14 = t03 ^ t10;
	t15 = *a   ^ *c  ;
	*d   = t14 ^ t13;
	t17 = t05 & t13;
	t18 = t14 | t17;
	*a   = t15 ^ t18;

}

#[allow(non_snake_case, unused_assignments)]
pub fn sbox_1(words: &mut [u32]) {
    let mut t02 = 0;
    let mut t03 = 0;
    let mut t04 = 0;
    let mut t05 = 0;
    let mut t06 = 0;
    let mut t07 = 0;
    let mut t08 = 0;
    let mut t10 = 0;
    let mut t11 = 0;
    let mut t12 = 0;
    let mut t13 = 0;
    let mut t16 = 0;
    let mut t17 = 0;
    let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *a   | *d  ;
	t02 = *c   ^ *d  ;
	t03 =     !*b  ;
	t04 = *a   ^ *c  ;
	t05 = *a   | t03;
	t06 = *d   & t04;
	t07 = t01 & t02;
	t08 = *b   | t06;
	*c   = t02 ^ t05;
	t10 = t07 ^ t08;
	t11 = t01 ^ t10;
	t12 = *c   ^ t11;
	t13 = *b   & *d  ;
	*d   =     !t10;
	*b   = t13 ^ t12;
	t16 = t10 | *b  ;
	t17 = t05 & t16;
	*a   = *c   ^ t17; 
}

#[allow(non_snake_case, unused_assignments)]
pub fn inv_sbox_1(words: &mut [u32]) {

    let mut t02= 0;
    let mut t03= 0;
    let mut t04= 0;
    let mut t05= 0;
    let mut t06= 0;
    let mut t07= 0;
    let mut t08= 0;
    let mut t09= 0;
    let mut t10= 0;
    let mut t11= 0;
    let mut t14= 0;
    let mut t15= 0;
    let mut t17= 0;
    let mut t01= 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *a   ^ *b  ;
	t02 = *b   | *d  ;
	t03 = *a   & *c  ;
	t04 = *c   ^ t02;
	t05 = *a   | t04;
	t06 = t01 & t05;
	t07 = *d   | t03;
	t08 = *b   ^ t06;
	t09 = t07 ^ t06;
	t10 = t04 | t03;
	t11 = *d   & t08;
	*c   =     !t09;
	*b   = t10 ^ t11;
	t14 = *a   | *c  ;
	t15 = t06 ^ *b  ;
	*d   = t01 ^ t04;
	t17 = *c   ^ t15;
	*a   = t14 ^ t17; 

}

#[allow(non_snake_case, unused_assignments)]
pub fn sbox_2(words: &mut [u32]) {

    let mut t02 = 0;
    let mut t03 = 0;
    let mut t05 = 0;
    let mut t06 = 0;
    let mut t07 = 0;
    let mut t08 = 0;
    let mut t09 = 0;
    let mut t10 = 0;
    let mut t12 = 0;
    let mut t13 = 0;
    let mut t14 = 0;
    let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

    t01 = *a   | *c  ;
    t02 = *a   ^ *b  ;
    t03 = *d   ^ t01;
    *a   = t02 ^ t03;
    t05 = *c   ^ *a  ;
    t06 = *b   ^ t05;
    t07 = *b   | t05;
    t08 = t01 & t06;
    t09 = t03 ^ t07;
    t10 = t02 | t09;
    *b   = t10 ^ t08;
    t12 = *a   | *d  ;
    t13 = t09 ^ *b  ;
    t14 = *b   ^ t13;
    *d   =     !t09;
    *c   = t12 ^ t14;

}

#[allow(non_snake_case, unused_assignments)]
pub fn inv_sbox_2(words: &mut [u32]) {

    let mut t02 = 0;
    let mut t03 = 0;
    let mut t04 = 0;
    let mut t06 = 0;
    let mut t07 = 0;
    let mut t08 = 0;
    let mut t09 = 0;
    let mut t10 = 0;
    let mut t11 = 0;
    let mut t12 = 0;
    let mut t15 = 0;
    let mut t16 = 0;
    let mut t17 = 0;
    let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

    t01 = *a   ^ *d  ;
    t02 = *c   ^ *d  ;
    t03 = *a   & *c  ;
    t04 = *b   | t02;
    *a   = t01 ^ t04;
    t06 = *a   | *c  ;
    t07 = *d   | *a  ;
    t08 =     !*d  ;
    t09 = *b   & t06;
    t10 = t08 | t03;
    t11 = *b   & t07;
    t12 = t06 & t02;
    *d   = t09 ^ t10;
    *b   = t12 ^ t11;
    t15 = *c   & *d  ;
    t16 = *a   ^ *b  ;
    t17 = t10 ^ t15;
    *c   = t16 ^ t17;
}

#[allow(non_snake_case, unused_assignments)]
pub fn sbox_3(words: &mut [u32]) {

    let mut t02 = 0;
    let mut t03 = 0;
    let mut t04 = 0;
    let mut t05 = 0;
    let mut t06 = 0;
    let mut t07 = 0;
    let mut t08 = 0;
    let mut t09 = 0;
    let mut t10 = 0;
    let mut t11 = 0;
    let mut t13 = 0;
    let mut t14 = 0;
    let mut t15 = 0;
    let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

   t01 = *a   ^ *c  ; 
   t02 = *a   | *d  ; 
   t03 = *a   & *d  ; 
   t04 = t01 & t02; 
   t05 = *b   | t03; 
   t06 = *a   & *b  ; 
   t07 = *d   ^ t04; 
   t08 = *c   | t06; 
   t09 = *b   ^ t07; 
   t10 = *d   & t05; 
   t11 = t02 ^ t10; 
   *d   = t08 ^ t09; 
   t13 = *d   | *d  ; 
   t14 = *a   | t07; 
   t15 = *b   & t13; 
   *c   = t08 ^ t11; 
   *a   = t14 ^ t15; 
   *b   = t05 ^ t04;
}

#[allow(non_snake_case, unused_assignments)]
pub fn inv_sbox_3(words: &mut [u32]) {
    let mut t02 = 0;
 	let mut t03 = 0;
 	let mut t04 = 0;
 	let mut t05 = 0;
 	let mut t06 = 0;
 	let mut t07 = 0;
 	let mut t09 = 0;
 	let mut t11 = 0;
 	let mut t12 = 0;
 	let mut t13 = 0;
 	let mut t14 = 0;
 	let mut t16 = 0;
 	let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *c   | *d  ; 
	t02 = *a   | *d  ; 
	t03 = *c   ^ t02; 
	t04 = *b   ^ t02; 
	t05 = *a   ^ *d  ; 
	t06 = t04 & t03; 
	t07 = *b   & t01; 
	*c   = t05 ^ t06; 
	t09 = *a   ^ t03; 
	*a   = t07 ^ t03; 
	t11 = *a   | t05; 
	t12 = t09 & t11; 
	t13 = *a   & *c  ; 
	t14 = t01 ^ t05; 
	*b   = *b   ^ t12; 
	t16 = *b   | t13; 
	*d   = t14 ^ t16;
}

#[allow(non_snake_case, unused_assignments)]
pub fn sbox_4(words: &mut [u32]) {
    let mut t02 = 0;
    let mut t03 = 0;
    let mut t04 = 0;
    let mut t05 = 0;
    let mut t06 = 0;
    let mut t08 = 0;
    let mut t09 = 0;
    let mut t10 = 0;
    let mut t11 = 0;
    let mut t12 = 0;
    let mut t13 = 0;
    let mut t14 = 0;
    let mut t15 = 0;
    let mut t16 = 0;
    let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

   t01 = *a   | *b  ; 
   t02 = *b   | *c  ; 
   t03 = *a   ^ t02; 
   t04 = *b   ^ *d  ; 
   t05 = *d   | t03; 
   t06 = *d   & t01; 
   *d   = t03 ^ t06; 
   t08 = *d   & t04; 
   t09 = t04 & t05; 
   t10 = *c   ^ t06; 
   t11 = *b   & *c  ; 
   t12 = t04 ^ t08; 
   t13 = t11 | t03; 
   t14 = t10 ^ t09; 
   t15 = *a   & t05; 
   t16 = t11 | t12; 
   *c   = t13 ^ t08; 
   *b   = t15 ^ t16; 
   *a   =     !t14;
}

#[allow(non_snake_case, unused_assignments)]
pub fn inv_sbox_4(words: &mut [u32]) {
    let mut t02 = 0;
 	let mut t03 = 0;
 	let mut t04 = 0;
 	let mut t05 = 0;
 	let mut t06 = 0;
 	let mut t07 = 0;
 	let mut t09 = 0;
 	let mut t10 = 0;
 	let mut t11 = 0;
 	let mut t12 = 0;
 	let mut t13 = 0;
 	let mut t15 = 0;
 	let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *b   | *d  ; 
	t02 = *c   | *d  ; 
	t03 = *a   & t01; 
	t04 = *b   ^ t02; 
	t05 = *c   ^ *d  ; 
	t06 =     !t03; 
	t07 = *a   & t04; 
	*b   = t05 ^ t07; 
	t09 = *b   | t06; 
	t10 = *a   ^ t07; 
	t11 = t01 ^ t09; 
	t12 = *d   ^ t04; 
	t13 = *c   | t10; 
	*d   = t03 ^ t12; 
	t15 = *a   ^ t04; 
	*c   = t11 ^ t13; 
	*a   = t15 ^ t09;
}

#[allow(non_snake_case, unused_assignments)]
pub fn sbox_5(words: &mut [u32]) {
    let mut t02 = 0;
 	let mut t03 = 0;
 	let mut t04 = 0;
 	let mut t05 = 0;
 	let mut t07 = 0;
 	let mut t08 = 0;
 	let mut t09 = 0;
 	let mut t10 = 0;
 	let mut t11 = 0;
 	let mut t12 = 0;
 	let mut t13 = 0;
 	let mut t14 = 0;
 	let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *b   ^ *d  ; 
	t02 = *b   | *d  ; 
	t03 = *a   & t01; 
	t04 = *c   ^ t02; 
	t05 = t03 ^ t04; 
	*a   =     !t05; 
	t07 = *a   ^ t01; 
	t08 = *d   | *a  ; 
	t09 = *b   | t05; 
	t10 = *d   ^ t08; 
	t11 = *b   | t07; 
	t12 = t03 | *a  ; 
	t13 = t07 | t10; 
	t14 = t01 ^ t11; 
	*c   = t09 ^ t13; 
	*b   = t07 ^ t08; 
	*d   = t12 ^ t14;
}

#[allow(non_snake_case, unused_assignments)]
pub fn inv_sbox_5(words: &mut [u32]) {
    let mut t02 = 0;
 	let mut t03 = 0;
 	let mut t04 = 0;
 	let mut t05 = 0;
 	let mut t07 = 0;
 	let mut t08 = 0;
 	let mut t09 = 0;
 	let mut t10 = 0;
 	let mut t12 = 0;
 	let mut t13 = 0;
 	let mut t15 = 0;
 	let mut t16 = 0;
 	let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *a   & *d  ; 
	t02 = *c   ^ t01; 
	t03 = *a   ^ *d  ; 
	t04 = *b   & t02; 
	t05 = *a   & *c  ; 
	*a   = t03 ^ t04; 
	t07 = *a   & *a  ; 
	t08 = t01 ^ *a  ; 
	t09 = *b   | t05; 
	t10 =     !*b  ; 
	*b   = t08 ^ t09; 
	t12 = t10 | t07; 
	t13 = *a   | *b  ; 
	*d   = t02 ^ t12; 
	t15 = t02 ^ t13; 
	t16 = *b   ^ *d  ; 
	*c   = t16 ^ t15; 
}

#[allow(non_snake_case, unused_assignments)]
pub fn sbox_6(words: &mut [u32]) {
    let mut t02 = 0;
 	let mut t03 = 0;
 	let mut t04 = 0;
 	let mut t05 = 0;
 	let mut t07 = 0;
 	let mut t08 = 0;
 	let mut t09 = 0;
 	let mut t10 = 0;
 	let mut t11 = 0;
 	let mut t12 = 0;
 	let mut t13 = 0;
 	let mut t15 = 0;
 	let mut t17 = 0;
 	let mut t18 = 0;
 	let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *a   & *d  ; 
	t02 = *b   ^ *c  ; 
	t03 = *a   ^ *d  ; 
	t04 = t01 ^ t02; 
	t05 = *b   | *c  ; 
	*b   =     !t04; 
	t07 = t03 & t05; 
	t08 = *b   & *b  ; 
	t09 = *a   | *c  ; 
	t10 = t07 ^ t08; 
	t11 = *b   | *d  ; 
	t12 = *c   ^ t11; 
	t13 = t09 ^ t10; 
	*c   =     !t13; 
	t15 = *b   & t03; 
	*d   = t12 ^ t07; 
	t17 = *a   ^ *b  ; 
	t18 = *c   ^ t15; 
	*a   = t17 ^ t18;
}

#[allow(non_snake_case, unused_assignments)]
pub fn inv_sbox_6(words: &mut [u32]) {
    let mut t02 = 0;
 	let mut t03 = 0;
 	let mut t04 = 0;
 	let mut t05 = 0;
 	let mut t06 = 0;
 	let mut t07 = 0;
 	let mut t08 = 0;
 	let mut t09 = 0;
 	let mut t12 = 0;
 	let mut t13 = 0;
 	let mut t14 = 0;
 	let mut t15 = 0;
 	let mut t16 = 0;
 	let mut t17 = 0;
 	let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *a   ^ *c  ; 
	t02 =     !*c  ; 
	t03 = *b   & t01; 
	t04 = *b   | t02; 
	t05 = *d   | t03; 
	t06 = *b   ^ *d  ; 
	t07 = *a   & t04; 
	t08 = *a   | t02; 
	t09 = t07 ^ t05; 
	*b   = t06 ^ t08; 
	*a   =     !t09; 
	t12 = *b   & *a  ; 
	t13 = t01 & t05; 
	t14 = t01 ^ t12; 
	t15 = t07 ^ t13; 
	t16 = *d   | t02; 
	t17 = *a   ^ *b  ; 
	*d   = t17 ^ t15; 
	*c   = t16 ^ t14;
}

#[allow(non_snake_case, unused_assignments)]
pub fn sbox_7(words: &mut [u32]) {
    let mut t02 = 0;
 	let mut t03 = 0;
 	let mut t04 = 0;
 	let mut t05 = 0;
 	let mut t06 = 0;
 	let mut t08 = 0;
 	let mut t09 = 0;
 	let mut t10 = 0;
 	let mut t11 = 0;
 	let mut t13 = 0;
 	let mut t14 = 0;
 	let mut t15 = 0;
 	let mut t16 = 0;
 	let mut t17 = 0;
 	let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *a   & *c  ; 
	t02 =     !*d  ; 
	t03 = *a   & t02; 
	t04 = *b   | t01; 
	t05 = *a   & *b  ; 
	t06 = *c   ^ t04; 
	*d   = t03 ^ t06; 
	t08 = *c   | *d  ; 
	t09 = *d   | t05; 
	t10 = *a   ^ t08; 
	t11 = t04 & *d  ; 
	*b   = t09 ^ t10; 
	t13 = *b   ^ *b  ; 
	t14 = t01 ^ *b  ; 
	t15 = *c   ^ t05; 
	t16 = t11 | t13; 
	t17 = t02 | t14; 
	*a   = t15 ^ t17; 
	*c   = *a   ^ t16;
}

#[allow(non_snake_case, unused_assignments)]
pub fn inv_sbox_7(words: &mut [u32]) {
    let mut t02 = 0;
 	let mut t03 = 0;
 	let mut t04 = 0;
 	let mut t06 = 0;
 	let mut t07 = 0;
 	let mut t08 = 0;
 	let mut t09 = 0;
 	let mut t10 = 0;
 	let mut t11 = 0;
 	let mut t13 = 0;
 	let mut t14 = 0;
 	let mut t15 = 0;
 	let mut t16 = 0;
 	let mut t01 = 0;
    let (a,b,c,d) = words_as_word_mut(words);

	t01 = *a   & *b  ; 
	t02 = *a   | *b  ; 
	t03 = *c   | t01; 
	t04 = *d   & t02; 
	*d   = t03 ^ t04; 
	t06 = *b   ^ t04; 
	t07 = *d   ^ *d  ; 
	t08 =     !t07; 
	t09 = t06 | t08; 
	t10 = *b   ^ *d  ; 
	t11 = *a   | *d  ; 
	*b   = *a   ^ t09; 
	t13 = *c   ^ t06; 
	t14 = *c   & t11; 
	t15 = *d   | *b  ; 
	t16 = t01 | t10; 
	*a   = t13 ^ t15; 
	*c   = t14 ^ t16; 
}



#[cfg(test)]
mod tests {

    use super::*;

	/* 
		SBOX Test values generated from Reference Implementation
		on floppy 2 of the AES submission package
	*/
    #[test]
	fn test_sbox0() {

		let mut words = [0,1,2,3];
		let expected = [0xFFFFFFFD, 0xFFFFFFFC, 0x00000001, 0x00000000, ];
		sbox_0(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_sbox1() {

		let mut words = [0,1,2,3];
		let expected = [0x00000001, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFD, ];
		sbox_1(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_sbox2() {

		let mut words = [0,1,2,3];
		let expected = [0x00000000, 0x00000001, 0x00000001, 0xFFFFFFFD, ];
		sbox_2(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_sbox3() {

		let mut words = [0,1,2,3];
		let expected = [0x00000001, 0x00000003, 0x00000000, 0x00000002, ];
		sbox_3(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_sbox4() {

		let mut words = [0,1,2,3];
		let expected = [0xFFFFFFFE, 0x00000000, 0x00000001, 0x00000002, ];
		sbox_4(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_sbox5() {

		let mut words = [0,1,2,3];
		let expected = [0xFFFFFFFE, 0x00000003, 0xFFFFFFFD, 0x00000001, ];
		sbox_5(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_sbox6() {

		let mut words = [0,1,2,3];
		let expected = [0xFFFFFFFE, 0xFFFFFFFC, 0x00000002, 0xFFFFFFFE, ];
		sbox_6(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_sbox7() {

		let mut words = [0,1,2,3];
		let expected = [0xFFFFFFFE, 0x00000000, 0xFFFFFFFF, 0x00000003, ];
		sbox_7(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_inv_sbox0() {

		let mut words = [0,1,2,3];
		let expected = [0x00000003, 0x00000002, 0xFFFFFFFF, 0xFFFFFFFE, ];
		inv_sbox_0(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_inv_sbox1() {

		let mut words = [0,1,2,3];
		let expected = [0x00000000, 0x00000001, 0xFFFFFFFD, 0x00000000, ];
		inv_sbox_1(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_inv_sbox2() {

		let mut words = [0,1,2,3];
		let expected = [0x00000002, 0x00000001, 0xFFFFFFFF, 0xFFFFFFFC, ];
		inv_sbox_2(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_inv_sbox3() {

		let mut words = [0,1,2,3];
		let expected = [0x00000000, 0x00000000, 0x00000003, 0x00000000, ];
		inv_sbox_3(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_inv_sbox4() {

		let mut words = [0,1,2,3];
		let expected = [0xFFFFFFFD, 0x00000001, 0xFFFFFFFE, 0x00000001, ];
		inv_sbox_4(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_inv_sbox5() {

		let mut words = [0,1,2,3];
		let expected = [0x00000003, 0x00000002, 0xFFFFFFFE, 0xFFFFFFFD, ];
		inv_sbox_5(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_inv_sbox6() {

		let mut words = [0,1,2,3];
		let expected = [0xFFFFFFFC, 0xFFFFFFFF, 0x00000001, 0x00000001, ];
		inv_sbox_6(&mut words);
		assert_eq!(words, expected);
	}

	#[test]
	fn test_inv_sbox7() {

		let mut words = [0,1,2,3];
		let expected = [0xFFFFFFFD, 0xFFFFFFFF, 0x00000000, 0x00000003, ];
		inv_sbox_7(&mut words);
		assert_eq!(words, expected);
	}
}
