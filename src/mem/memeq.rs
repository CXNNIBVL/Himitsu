
/// Safe comparison of Byte buffers
/// * 'a' - First Buffer
/// * 'b' - Second Buffer
#[inline(never)]
pub fn memeq_s(a: &[u8], b: &[u8]) -> bool {

    if a.len() != b.len() { return false; }

    a.iter().zip(b.iter())
    .map(|(x, y)| x ^ y)
    .fold(0, |sum, nx | sum | nx)
    .eq(&0)
}

#[cfg(test)]
mod tests {

    use super::memeq_s;

    // Compare two equal buffers
    #[test]
    fn a_eq_b() {

        let a: Vec<u8> = vec![1, 2, 3, 4];
        let b = a.clone();

        assert!(memeq_s(&a, &b))
    }

    // Compare two unequal buffers
    #[test]
    fn a_uneq_b() {
        let a = vec![1, 2, 3, 4];
        let b = vec![4, 2, 3, 4];

        assert!(!memeq_s(&a,&b))
    }

    // Compare two buffers that differ in length
    #[test]
    fn a_longer_b() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![1, 2, 3, 4];

        assert!(!memeq_s(&a,&b))
    }
}