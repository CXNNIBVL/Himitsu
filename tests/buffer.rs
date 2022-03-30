#[cfg(test)]
mod tests {

    use himitsu::util::buffer::FixedBuffer;

    #[test]
    fn test_push() {
        let mut buf_size_4: FixedBuffer<u8, 4> = FixedBuffer::new();

        let data = vec![1u8, 2, 3, 4];
        let data_iter = data.iter();

        for b in data_iter {
            assert!(buf_size_4.push_ref(b));
        }

        assert!(!buf_size_4.push_ref(&12u8))
    }

    #[test]
    fn test_extract() {
        let mut buf: FixedBuffer<u8, 4> = FixedBuffer::new();
        let data = vec![1, 2, 3, 4];
        let exp = vec![1, 2, 3, 4];

        let data_iter = data.iter();

        for b in data_iter {
            buf.push_ref(b);
        }

        assert_eq!(exp, buf.extract());
    }

    #[test]
    fn test_index() {
        let mut buf: FixedBuffer<u8, 4> = FixedBuffer::new();
        let data = vec![1, 2, 3, 4];

        for el in data.iter() {
            buf.push_ref(el);
        }

        assert_eq!(2, buf[1]);

        buf[1] = 55;
        assert_eq!(55, buf[1]);
    }
}
