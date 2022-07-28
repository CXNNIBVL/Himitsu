#[cfg(test)]
mod tests {

    use himitsu::util::memeq_s;

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

        assert!(!memeq_s(&a, &b))
    }

    // Compare two buffers that differ in length
    #[test]
    fn a_longer_b() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![1, 2, 3, 4];

        assert!(!memeq_s(&a, &b))
    }
}
