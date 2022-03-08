use std::ops::{Index, IndexMut};
use std::convert::{AsMut, AsRef};
use std::mem;


#[derive(Clone, Copy, Debug)]
pub struct FixedBuffer<T, const BLOCKSIZE: usize> 
    where T: Clone + Copy + Default
{
    buf: [T; BLOCKSIZE],
    capacity: usize,
}

impl<T, const B: usize> FixedBuffer<T, B> 
    where T: Clone + Copy + Default
{
    /// Create a new buffer
    pub fn new() -> Self {
        Self {
            buf: [T::default(); B],
            capacity: B,
        }
    }

    /// Returns the length of the buffer
    pub fn len(&self) -> usize { B }

    /// Pushes an element into a buffer. Returns a bool whether the operation was successful
    pub fn push_ref(&mut self, element: &T) -> bool {
        if self.capacity == 0 { return false; }

        let position = self.len() - self.capacity;
        self.buf[position] = element.clone();
        self.capacity -= 1;
        true
    }

    /// Pushes an element into a buffer. Returns a bool whether the operation was successful
    pub fn push(&mut self, element: T) -> bool {
        if self.capacity == 0 { return false; }

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
    pub fn is_full(&self) -> bool { self.capacity() == 0 }

    /// Provides fast overriding of the buffers elements.
    /// This does not update the buffers capacity, so this function should only be called
    /// for immediate processing afterwards.
    pub fn override_contents(&mut self, slice: &[T], x: usize) -> usize {
        let mut ix = 0;

        while ix < slice.len() && ix < x && ix < self.len() {
            self[ix] = slice[ix];
            ix += 1;
        }

        ix
    }

    /// Extract the buffers contents and resets the buffer
    pub fn extract(&mut self) -> [T; B] {
        self.capacity = B;
        mem::replace(&mut self.buf, [T::default(); B])
    }
}

impl<T, const B: usize> Index<usize> for FixedBuffer<T, B> 
    where T: Clone + Copy + Default
{

    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<T, const B: usize> IndexMut<usize> for FixedBuffer<T, B> 
    where T: Clone + Copy + Default
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

impl<T, const B: usize> AsRef<[T; B]> for FixedBuffer<T, B> 
    where T: Clone + Copy + Default,
{
    fn as_ref(&self) -> &[T;B] {
        &self.buf
    }
}

impl<T, const B: usize> AsMut<[T; B]> for FixedBuffer<T, B> 
    where T: Clone + Copy + Default,
{
    fn as_mut(&mut self) -> &mut [T;B] {
        &mut self.buf
    }
}

impl<T, const B: usize> IntoIterator for FixedBuffer<T, B> 
    where T: Clone + Copy + Default
{
    type Item = T;
    type IntoIter = std::array::IntoIter<T, B>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.buf)
    }
}

impl<'a, T, const B: usize> IntoIterator for &'a FixedBuffer<T, B> 
    where T: Clone + Copy + Default
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buf.iter()
    }
}

impl<'a, T, const B: usize> IntoIterator for &'a mut FixedBuffer<T, B> 
    where T: Clone + Copy + Default
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buf.iter_mut()
    }
}

impl<T: Clone + Copy + Default, const B: usize> From<[T;B]> for FixedBuffer<T, B> {

    /// Create a new filled buffer from an array of same type and length
    fn from(buf: [T;B]) -> Self {
        Self { buf, capacity: 0 }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_push() {

        let mut buf_size_4: FixedBuffer<u8, 4> = FixedBuffer::new();

        let data = vec![1u8, 2, 3, 4];
        let data_iter = data.iter();

        for b in data_iter {
            assert!(buf_size_4.push_ref(b));
        }

        assert!(!buf_size_4.push_ref(&12u8))
    }

    #[test]
    fn test_extract() {
        let mut buf: FixedBuffer<u8, 4> = FixedBuffer::new();
        let data = vec![1, 2, 3, 4];
        let exp = vec![1,2,3,4];

        let data_iter = data.iter();

        for b in data_iter {
            buf.push_ref(b);
        }

        assert_eq!(exp, buf.extract());
    }

    #[test]
    fn test_index() {
        let mut buf: FixedBuffer<u8, 4> = FixedBuffer::new();
        let data = vec![1, 2, 3, 4];
        
        for el in data.iter() {
            buf.push_ref(el);
        }

        assert_eq!(2, buf[1]);

        buf[1] = 55;
        assert_eq!(55, buf[1]);
    }

}