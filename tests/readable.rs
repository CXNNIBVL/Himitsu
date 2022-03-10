#[cfg(test)]
mod tests {

    use std::io::Read;

    use himitsu::util::readable::Readable;

    #[test]
    fn test_readable() {

        let data = vec![1 , 2, 3, 4];
        let mut rdb: Readable<Vec<u8>> = Readable::new(data.clone());
        let mut out = Vec::new();

        rdb.read_to_end(&mut out).unwrap();

        assert_eq!(data, out);
    }
}