pub use conversion::*;
pub use index::*;
pub use iter::*;
use std::mem;

#[derive(Clone, Copy, Debug)]
pub struct FixedBuffer<T, const BLOCKSIZE: usize>
where
    T: Clone + Copy + Default,
{
    buf: [T; BLOCKSIZE],
    capacity: usize,
}

impl<T, const B: usize> FixedBuffer<T, B>
where
    T: Clone + Copy + Default,
{
    /// Create a new buffer
    pub fn new() -> Self {
        Self {
            buf: [T::default(); B],
            capacity: B,
        }
    }

    /// Returns the length of the buffer
    pub fn len(&self) -> usize {
        B
    }

    pub fn as_slice(&self) -> &[T] {
        &self.buf
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.buf
    }

    /// Pushes an element into a buffer. Returns a bool whether the operation was successful
    pub fn push_ref(&mut self, element: &T) -> bool {
        if self.capacity == 0 {
            return false;
        }

        let position = self.len() - self.capacity;
        self.buf[position] = element.clone();
        self.capacity -= 1;
        true
    }

    /// Pushes an element into a buffer. Returns a bool whether the operation was successful
    pub fn push(&mut self, element: T) -> bool {
        if self.capacity == 0 {
            return false;
        }

        let position = self.len() - self.capacity;
        self.buf[position] = element;
        self.capacity -= 1;
        true
    }

    /// Pushes a slice into the buffer. Returns the number of elements successfully pushed
    pub fn push_slice(&mut self, slice: &[T]) -> usize {
        let mut pushed = 0;

        while self.capacity() != 0 && pushed < slice.len() {
            self.push_ref(&slice[pushed]);
            pushed += 1;
        }

        pushed
    }

    /// Returns the capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns a bool indicating whether the buffer is filled
    pub fn is_full(&self) -> bool {
        self.capacity() == 0
    }
    pub fn is_empty(&self) -> bool {
        self.capacity() == B
    }

    /// Extract the buffers contents and resets the buffer
    pub fn extract(&mut self) -> [T; B] {
        self.capacity = B;
        mem::replace(&mut self.buf, [T::default(); B])
    }

    /// Extract the buffers contents and resets the buffer in place, not resetting the capacity
    pub fn extract_in_place(&mut self, buf: [T; B]) -> [T; B] {
        mem::replace(&mut self.buf, buf)
    }
}

mod index {
    use super::*;
    use std::ops::{Index, IndexMut};
    use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

    impl<T, const B: usize> Index<usize> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.buf[index]
        }
    }

    impl<T, const B: usize> IndexMut<usize> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.buf[index]
        }
    }

    impl<T, const B: usize> Index<Range<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Output = [T];

        fn index<'a>(&'a self, index: Range<usize>) -> &'a Self::Output {
            &self.buf[index]
        }
    }

    impl<T, const B: usize> IndexMut<Range<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn index_mut<'a>(&'a mut self, index: Range<usize>) -> &'a mut Self::Output {
            &mut self.buf[index]
        }
    }

    impl<T, const B: usize> Index<RangeFrom<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Output = [T];

        fn index<'a>(&'a self, index: RangeFrom<usize>) -> &'a Self::Output {
            &self.buf[index]
        }
    }

    impl<T, const B: usize> IndexMut<RangeFrom<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn index_mut<'a>(&'a mut self, index: RangeFrom<usize>) -> &'a mut Self::Output {
            &mut self.buf[index]
        }
    }

    impl<T, const B: usize> Index<RangeFull> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Output = [T];

        fn index<'a>(&'a self, index: RangeFull) -> &'a Self::Output {
            &self.buf[index]
        }
    }

    impl<T, const B: usize> IndexMut<RangeFull> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn index_mut<'a>(&'a mut self, index: RangeFull) -> &'a mut Self::Output {
            &mut self.buf[index]
        }
    }

    impl<T, const B: usize> Index<RangeInclusive<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Output = [T];

        fn index<'a>(&'a self, index: RangeInclusive<usize>) -> &'a Self::Output {
            &self.buf[index]
        }
    }

    impl<T, const B: usize> IndexMut<RangeInclusive<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn index_mut<'a>(&'a mut self, index: RangeInclusive<usize>) -> &'a mut Self::Output {
            &mut self.buf[index]
        }
    }

    impl<T, const B: usize> Index<RangeTo<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Output = [T];

        fn index<'a>(&'a self, index: RangeTo<usize>) -> &'a Self::Output {
            &self.buf[index]
        }
    }

    impl<T, const B: usize> IndexMut<RangeTo<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn index_mut<'a>(&'a mut self, index: RangeTo<usize>) -> &'a mut Self::Output {
            &mut self.buf[index]
        }
    }

    impl<T, const B: usize> Index<RangeToInclusive<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Output = [T];

        fn index<'a>(&'a self, index: RangeToInclusive<usize>) -> &'a Self::Output {
            &self.buf[index]
        }
    }

    impl<T, const B: usize> IndexMut<RangeToInclusive<usize>> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn index_mut<'a>(&'a mut self, index: RangeToInclusive<usize>) -> &'a mut Self::Output {
            &mut self.buf[index]
        }
    }
}

mod conversion {
    use super::*;
    use std::convert::{AsMut, AsRef};

    impl<T, const B: usize> AsRef<[T; B]> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn as_ref(&self) -> &[T; B] {
            &self.buf
        }
    }

    impl<T, const B: usize> AsMut<[T; B]> for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        fn as_mut(&mut self) -> &mut [T; B] {
            &mut self.buf
        }
    }

    impl<T: Clone + Copy + Default, const B: usize> From<[T; B]> for FixedBuffer<T, B> {
        /// Create a new filled buffer from an array of same type and length
        fn from(buf: [T; B]) -> Self {
            Self { buf, capacity: 0 }
        }
    }

    impl<'a, T: Clone + Copy + Default, const B: usize> From<&'a [T; B]> for FixedBuffer<T, B> {
        fn from(buf: &'a [T; B]) -> Self {
            Self {
                buf: buf.clone(),
                capacity: 0,
            }
        }
    }

    impl<'a, T: Clone + Copy + Default, const B: usize> From<&'a mut [T; B]> for FixedBuffer<T, B> {
        fn from(buf: &'a mut [T; B]) -> Self {
            Self {
                buf: buf.clone(),
                capacity: 0,
            }
        }
    }

    impl<T: Clone + Copy + Default, const B: usize> From<FixedBuffer<T, B>> for [T; B] {
        fn from(buf: FixedBuffer<T, B>) -> [T; B] {
            buf.buf
        }
    }
}

mod iter {
    use super::*;
    impl<T, const B: usize> IntoIterator for FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Item = T;
        type IntoIter = std::array::IntoIter<T, B>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIterator::into_iter(self.buf)
        }
    }

    impl<'a, T, const B: usize> IntoIterator for &'a FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.buf.iter()
        }
    }

    impl<'a, T, const B: usize> IntoIterator for &'a mut FixedBuffer<T, B>
    where
        T: Clone + Copy + Default,
    {
        type Item = &'a mut T;
        type IntoIter = std::slice::IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.buf.iter_mut()
        }
    }
}
