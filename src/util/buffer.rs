use crate::traits::util::buffer::Buffer;
pub use conversion::*;
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug)]
pub struct ArrayBuffer<T, const S: usize>
where
    T: Clone + Copy + Default,
{
    buf: [T; S],
    capacity: usize,
}

impl<T, const S: usize> Buffer<T> for ArrayBuffer<T, S>
where
    T: Clone + Copy + Default,
{
    fn buffer(&self) -> &[T] {
        &self.buf
    }

    fn buffer_mut(&mut self) -> &mut [T] {
        &mut self.buf
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

impl<T, const S: usize> ArrayBuffer<T, S>
where
    T: Clone + Copy + Default,
{
    /// Create a new buffer
    pub fn new() -> Self {
        Self {
            buf: [T::default(); S],
            capacity: S,
        }
    }

    /// Extract the buffers contents and resets the buffer
    pub fn extract(&mut self) -> [T; S] {
        self.capacity = S;
        mem::replace(&mut self.buf, [T::default(); S])
    }

    /// Extract the buffers contents and resets the buffer in place, not resetting the capacity
    pub fn extract_in_place(&mut self, buf: [T; S]) -> [T; S] {
        self.capacity = 0;
        mem::replace(&mut self.buf, buf)
    }
}

impl<T, const S: usize> Default for ArrayBuffer<T, S>
where
    T: Clone + Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const S: usize> Deref for ArrayBuffer<T, S>
where
    T: Clone + Copy + Default,
{
    type Target = [T; S];
    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<T, const S: usize> DerefMut for ArrayBuffer<T, S>
where
    T: Clone + Copy + Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

mod conversion {
    use super::*;

    impl<T: Clone + Copy + Default, const B: usize> From<[T; B]> for ArrayBuffer<T, B> {
        /// Create a new filled buffer from an array of same type and length
        fn from(buf: [T; B]) -> Self {
            Self { buf, capacity: 0 }
        }
    }

    impl<'a, T: Clone + Copy + Default, const B: usize> From<&'a [T; B]> for ArrayBuffer<T, B> {
        fn from(buf: &'a [T; B]) -> Self {
            Self {
                buf: buf.clone(),
                capacity: 0,
            }
        }
    }

    impl<'a, T: Clone + Copy + Default, const B: usize> From<&'a mut [T; B]> for ArrayBuffer<T, B> {
        fn from(buf: &'a mut [T; B]) -> Self {
            Self {
                buf: buf.clone(),
                capacity: 0,
            }
        }
    }

    impl<T: Clone + Copy + Default, const B: usize> From<ArrayBuffer<T, B>> for [T; B] {
        fn from(buf: ArrayBuffer<T, B>) -> [T; B] {
            buf.buf
        }
    }
}
