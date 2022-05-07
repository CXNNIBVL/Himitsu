use super::Array;
use crate::traits::util::buffer::Buffer;
pub use conversion::*;
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ArrayBuffer<T: Default, const S: usize> {
    buf: Array<T, S>,
    capacity: usize,
}

impl<T: Default + Clone, const S: usize> Buffer<T> for ArrayBuffer<T, S> {
    fn buffer(&self) -> &[T] {
        self.buf.as_ref()
    }

    fn buffer_mut(&mut self) -> &mut [T] {
        self.buf.as_mut()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn push(&mut self, element: T) -> bool {
        if self.capacity == 0 {
            return false;
        }

        let position = self.len() - self.capacity;
        self.buf[position] = element;
        self.capacity -= 1;
        true
    }
}

impl<T: Default + Copy, const S: usize> ArrayBuffer<T, S> {
    /// Create a new buffer
    pub fn new() -> Self {
        Self {
            buf: Array::default(),
            capacity: S,
        }
    }

    /// Extract the buffers contents and resets the buffer
    pub fn extract(&mut self) -> Array<T, S> {
        self.capacity = S;
        mem::take(&mut self.buf)
    }

    /// Extract the buffers contents and resets the buffer in place, not resetting the capacity
    pub fn extract_in_place(&mut self, buf: Array<T, S>) -> Array<T, S> {
        self.capacity = 0;
        mem::replace(&mut self.buf, buf)
    }
}

impl<T: Default + Copy, const S: usize> Default for ArrayBuffer<T, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default, const S: usize> Deref for ArrayBuffer<T, S> {
    type Target = [T; S];
    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<T: Default, const S: usize> DerefMut for ArrayBuffer<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

mod conversion {
    use super::*;

    impl<T: Default, const B: usize> From<[T; B]> for ArrayBuffer<T, B> {
        /// Create a new filled buffer from an array of same type and length
        fn from(buf: [T; B]) -> Self {
            Self {
                buf: Array::from(buf),
                capacity: 0,
            }
        }
    }

    impl<'a, T: Default + Clone, const B: usize> From<&'a [T; B]> for ArrayBuffer<T, B> {
        fn from(buf: &'a [T; B]) -> Self {
            Self {
                buf: Array::from(buf.clone()),
                capacity: 0,
            }
        }
    }

    impl<'a, T: Default + Clone, const B: usize> From<&'a mut [T; B]> for ArrayBuffer<T, B> {
        fn from(buf: &'a mut [T; B]) -> Self {
            Self {
                buf: Array::from(buf.clone()),
                capacity: 0,
            }
        }
    }

    impl<T: Default + Copy, const B: usize> From<ArrayBuffer<T, B>> for [T; B] {
        fn from(buf: ArrayBuffer<T, B>) -> [T; B] {
            buf.buf.into()
        }
    }
}
