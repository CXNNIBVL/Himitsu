pub trait Buffer<T> {

    type Output;

    /// Returns the size of the buffer
    fn len(&self) -> usize;

    /// Pushes an element into the buffer. Returns a bool whether the operation was successful
    fn push(&mut self, element: &T) -> bool;

    /// Pushes a slice into the buffer. Returns the number of elements successfully pushed
    fn push_slice(&mut self, slice: &[T]) -> usize {
        let mut pushed = 0;

        while self.capacity() != 0 && pushed < slice.len() {
            self.push(&slice[pushed]);
            pushed += 1;
        }

        pushed
    }

    /// Returns the buffers current capacity
    fn capacity(&self) -> usize;

    /// Returns whether the buffer is filled
    fn is_full(&self) -> bool { self.capacity() == 0 }

    fn as_slice(&self) -> &[T];

    fn as_slice_mut(&mut self) -> &mut [T];

    /// Attempts to overrides the first x elements of Buffer with contents of slice,
    /// returning the number of elements successfully overwritten
    /// 
    /// This operation does not update the buffers capacity, thus,
    /// a following push may override already overridden elements
    fn override_contents(&mut self, slice: &[T], x: usize) -> usize;

    /// Extracts the buffers content and resets it
    fn extract(&mut self) -> Self::Output;
}