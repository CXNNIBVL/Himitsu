use crate::mem;
use std::borrow::{Borrow, BorrowMut};
use std::convert::{AsMut, AsRef};
use std::ops::{Deref, DerefMut, Drop};

pub struct Vector<T: Default> {
    inner: Vec<T>,
    do_zero: bool,
}

impl<T: Default> Vector<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Default> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Default> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Default> Borrow<Vec<T>> for Vector<T> {
    fn borrow(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T: Default> BorrowMut<Vec<T>> for Vector<T> {
    fn borrow_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }
}

impl<T: Default> AsRef<Vec<T>> for Vector<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T: Default> AsMut<Vec<T>> for Vector<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }
}

impl<T: Default> From<Vec<T>> for Vector<T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            inner: v,
            do_zero: true,
        }
    }
}

impl<T: Default> From<Vector<T>> for Vec<T> {
    fn from(mut v: Vector<T>) -> Self {
        let vec = std::mem::replace(&mut v.inner, Vec::default());
        v.do_zero = false;
        vec
    }
}

impl<T: Default> Drop for Vector<T> {
    fn drop(&mut self) {
        mem::zeroize(&mut self.inner);
    }
}

impl<T: Default> Default for Vector<T> {
    fn default() -> Self {
        Self {
            inner: Vec::default(),
            do_zero: true,
        }
    }
}

impl<T: Default + Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            do_zero: true,
        }
    }
}

impl<T: Default> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        let vec = std::mem::replace(&mut self.inner, Vec::default());
        self.do_zero = false;
        IntoIterator::into_iter(vec)
    }
}
