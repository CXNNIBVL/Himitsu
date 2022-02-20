use std::ops::{Index, IndexMut};

pub trait Buffer<T>: Index<usize> + IndexMut<usize> {

    /// Create a new instance
    fn new() -> Self;

    /// Returns the size of the buffer
    fn size(&self) -> usize;

    /// Pushes an element into the buffer. Returns a bool whether the operation was successful
    fn push(&mut self, element: &T) -> bool;

    /// Returns the number of missing elements
    fn missing(&self) -> usize;

    /// Returns whether the buffer is filled
    fn is_full(&self) -> bool { self.missing() == 0 }

    /// Extracts the buffers content and resets it
    fn extract(&mut self) -> Vec<T>;
}