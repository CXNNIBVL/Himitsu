use crate::mem::Zeroize;

pub type SecureVec<T> = Vec<Zeroize<T>>;
pub type SecureArray<T, const S: usize> = [Zeroize<T>; S];