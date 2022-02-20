use std::mem;
pub use crate::traits::buffer::Buffer;
use std::ops::{Index, IndexMut};

pub struct FixedBuffer<T, const BLOCKSIZE: usize> 
    where T: Clone + Copy + Default
{
    buf: [T; BLOCKSIZE],
    capacity: usize
}

impl<T: Clone + Copy + Default, const B: usize> Buffer<T> for FixedBuffer<T, B> {

    fn new() -> Self {
        Self {
            buf: [T::default(); B],
            capacity: B
        }
    }

    fn size(&self) -> usize { B }

    fn push(&mut self, element: &T) -> bool {
        if self.capacity == 0 { return false; }

        let position = self.size() - self.capacity;
        self.buf[position] = element.clone();
        self.capacity -= 1;
        true
    }

    fn missing(&self) -> usize {
        self.capacity
    }


    fn extract(&mut self) -> Vec<T> {
        self.capacity = B;
        mem::replace(&mut self.buf, [T::default(); B]).to_vec()
    }
}

impl<T: Clone + Copy + Default, const B: usize> Index<usize> for FixedBuffer<T, B> {

    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<T: Clone + Copy + Default, const B: usize> IndexMut<usize> for FixedBuffer<T, B> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push() {

        let mut buf_size_4: FixedBuffer<u8, 4> = FixedBuffer::new();

        let data = vec![1, 2, 3, 4];
        let data_iter = data.iter();

        for b in data_iter {
            assert!(buf_size_4.push(b));
        }

        assert!(!buf_size_4.push(&12u8))
    }

    #[test]
    fn extract() {
        let mut buf: FixedBuffer<u8, 4> = FixedBuffer::new();
        let data = vec![1, 2, 3, 4];
        let exp = vec![1,2,3,4];

        let data_iter = data.iter();

        for b in data_iter {
            let _ = buf.push(b);
        }

        assert_eq!(exp, buf.extract());
    }

    #[test]
    fn index() {
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