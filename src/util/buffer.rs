use std::ops::{Index, IndexMut};
use std::convert::{AsMut, AsRef};
use std::mem;
use crate::traits::buffer::Buffer;


#[derive(Clone, Copy)]
pub struct FixedBuffer<T, const BLOCKSIZE: usize> 
    where T: Clone + Copy + Default
{
    buf: [T; BLOCKSIZE],
    capacity: usize,
}

impl<T, const B: usize> FixedBuffer<T, B> 
    where T: Clone + Copy + Default
{
    pub fn new() -> Self {
        Self {
            buf: [T::default(); B],
            capacity: B,
        }
    }
}

impl<T, const B: usize> Buffer<T> for FixedBuffer<T, B> 
    where T: Clone + Copy + Default,
{
    type Output = [T;B];

    fn len(&self) -> usize { B }

    fn push(&mut self, element: &T) -> bool {
        if self.capacity == 0 { return false; }

        let position = self.len() - self.capacity;
        self.buf[position] = element.clone();
        self.capacity -= 1;
        true
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn as_slice(&self) -> &[T] {
        self.as_ref()
    }

    fn as_slice_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }

    fn override_contents(&mut self, slice: &[T], x: usize) -> usize {
        let mut ix = 0;

        while ix < slice.len() && ix < x && ix < self.len() {
            self[ix] = slice[ix];
            ix += 1;
        }

        ix
    }


    fn extract(&mut self) -> Self::Output {
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
    type IntoIter = std::array::IntoIter<T,B>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new(self.buf)
    }
}

impl<T: Clone + Copy + Default, const B: usize> From<[T;B]> for FixedBuffer<T, B> {

    /// Create a new filled buffer from an array
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
            assert!(buf_size_4.push(b));
        }

        assert!(!buf_size_4.push(&12u8))
    }

    #[test]
    fn test_extract() {
        let mut buf: FixedBuffer<u8, 4> = FixedBuffer::new();
        let data = vec![1, 2, 3, 4];
        let exp = vec![1,2,3,4];

        let data_iter = data.iter();

        for b in data_iter {
            buf.push(b);
        }

        assert_eq!(exp, buf.extract());
    }

    #[test]
    fn test_index() {
        let mut buf: FixedBuffer<u8, 4> = FixedBuffer::new();
        let data = vec![1, 2, 3, 4];
        
        for el in data.iter() {
            buf.push(el);
        }

        assert_eq!(2, buf[1]);

        buf[1] = 55;
        assert_eq!(55, buf[1]);
    }

}