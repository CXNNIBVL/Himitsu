
pub fn zeroize<T: Default>(x: &mut [T]) {
    for element in x {
        *element = T::default()
    }
}