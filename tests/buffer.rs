#[cfg(test)]
mod tests {

    use himitsu::util::buffer::ArrayBuffer;
    use himitsu::traits::util::buffer::Buffer;

    #[test]
    fn test_push() {
        let mut buf_size_4: ArrayBuffer<u8, 4> = ArrayBuffer::new();

        let data = vec![1u8, 2, 3, 4];

        for b in data {
            assert!(buf_size_4.push(b));
        }

        assert!(!buf_size_4.push(12u8))
    }

    #[test]
    fn test_extract() {
        let mut buf: ArrayBuffer<u8, 4> = ArrayBuffer::new();
        let data = vec![1, 2, 3, 4];
        let exp = vec![1, 2, 3, 4];

        for b in data {
            buf.push(b);
        }

        assert_eq!(exp, buf.extract());
    }

    #[test]
    fn test_index() {
        let mut buf: ArrayBuffer<u8, 4> = ArrayBuffer::new();
        let data = vec![1, 2, 3, 4];

        for el in data {
            buf.push(el);
        }

        assert_eq!(2, buf[1]);

        buf[1] = 55;
        assert_eq!(55, buf[1]);
    }
}
