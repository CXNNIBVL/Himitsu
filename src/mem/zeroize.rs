use std::ops::{Drop, Deref, DerefMut};

pub fn zeroize<T: Default>(x: &mut [T]) {
    for element in x {
        *element = T::default()
    }
}

pub struct Zeroize<T: Default>(T);

impl<T: Default> Drop for Zeroize<T> {
    fn drop(&mut self) {
        self.0 = T::default();
    }
}

impl<T: Default> Deref for Zeroize<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default> DerefMut for Zeroize<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}