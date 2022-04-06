pub trait Buffer<T>
    where
        T: Clone
{
    fn buffer(&self) -> &[T];
    fn buffer_mut(&mut self) -> &mut [T];
    
    /// Returns the length of the buffer
    fn len(&self) -> usize {
        self.buffer().len()
    }

    /// Returns the capacity
    fn capacity(&self) -> usize;

    /// Returns a bool indicating whether the buffer is filled
    fn is_full(&self) -> bool {
        self.capacity() == 0
    }

    fn is_empty(&self) -> bool {
        self.capacity() == self.len()
    }

    /// Pushes an element into a buffer. Returns a bool whether the operation was successful
    fn push(&mut self, element: T) -> bool;

    /// Pushes a slice into the buffer. Returns the number of elements successfully pushed
    fn push_slice(&mut self, slice: &[T]) -> usize {
        let mut pushed = 0;

        while !self.is_full() && pushed < slice.len() {
            self.push(slice[pushed].clone());
            pushed += 1;
        }

        pushed
    }
}