use std::convert::{AsRef, AsMut};

pub struct Buffer<'a, T> {
    buffer: &'a mut [T],
    capacity: usize
}

impl<'a, T> Buffer<'a, T> {

    /// Create a new buffer
    pub fn new(buffer: &'a mut [T]) -> Self {
        Self { capacity: buffer.len(), buffer }
    }

    /// Get the capacity of the buffer
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the underlying buffers length
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_full(&self) -> bool {
        self.capacity() == 0
    }

    pub fn is_empty(&self) -> bool {
        !self.is_full()
    }

    /// Tries to push an element into the buffer.
    /// If the buffer runs out of space, it will return
    /// the element as the content of the Some state.
    /// Else, an None will be returned.
    pub fn push(&mut self, element: T) -> Option<T> {
        if self.capacity() == 0 {
            return Some(element)
        }

        let pos = self.len() - self.capacity();
        self.buffer[pos] = element;
        self.capacity -= 1;

        None
    }
}

impl<'a, T> AsRef<[T]> for Buffer<'a, T> {
    fn as_ref(&self) -> &[T] { self.buffer }
}

impl<'a, T> AsMut<[T]> for Buffer<'a, T> {
    fn as_mut(&mut self) -> &mut [T] { self.buffer }
}
