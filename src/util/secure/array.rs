use std::ops::{Deref, DerefMut, Drop};
use std::convert::{AsRef, AsMut};
use std::borrow::{Borrow, BorrowMut};
use crate::mem;

#[derive(Debug)]
pub struct Array<T: Default, const S: usize> {
    inner: [T; S],
    do_zero: bool
}

impl<T: Default + Copy, const S: usize> Array<T, S> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Default, const S: usize> Deref for Array<T, S> {
    type Target = [T; S];
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Default, const S: usize> DerefMut for Array<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Default, const S: usize> Borrow<[T;S]> for Array<T, S> {
    fn borrow(&self) -> &[T;S] {
        &self.inner
    }
}

impl<T: Default, const S: usize> BorrowMut<[T;S]> for Array<T, S> {
    fn borrow_mut(&mut self) -> &mut [T;S] {
        &mut self.inner
    }
}

impl<T: Default, const S: usize> AsRef<[T;S]> for Array<T, S> {
    fn as_ref(&self) -> &[T;S] {
        &self.inner
    }
}

impl<T: Default, const S: usize> AsMut<[T;S]> for Array<T, S> {
    fn as_mut(&mut self) -> &mut [T;S] {
        &mut self.inner
    }
}

impl<T: Default, const S: usize> From<[T; S]> for Array<T, S> {
    fn from(v: [T; S]) -> Self {
        Self { inner: v, do_zero: true }
    }
}

impl<T: Default + Copy, const S: usize> From<Array<T,S>> for [T;S] {
    fn from(mut v: Array<T,S>) -> Self {
        let arr = std::mem::replace(&mut v.inner, [T::default(); S]);
        v.do_zero = false;
        arr
    }
}

impl<T: Default, const S: usize> Drop for Array<T, S> {
    fn drop(&mut self) {
        if self.do_zero {
            mem::zeroize(&mut self.inner);
        }
    }
}

impl<T: Default + Copy, const S: usize> Default for Array<T, S> {
    fn default() -> Self {
        Self { inner: [T::default(); S], do_zero: false }
    }
}

impl<T: Default + Clone, const S: usize> Clone for Array<T, S> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone(), do_zero: false }
    }
}

impl<T: Default + Copy, const S: usize> IntoIterator for Array<T, S> {
    type Item = T;
    type IntoIter = <[T;S] as IntoIterator>::IntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        let arr = std::mem::replace(&mut self.inner, [T::default(); S]);
        self.do_zero = false;
        IntoIterator::into_iter(arr)
    }
}