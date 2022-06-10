#[cfg(test)]
mod tests {

    use himitsu::encode::base64::Base64Encoder;
    use himitsu::errors::base64::Base64Error;

    // Encode some data that results in a Base64 String with 0 padding characters
    #[test]
    fn encode_basic_zero_pad() {
        let data = "aaa";
        let r = Base64Encoder::default().encode(data.as_bytes());
        assert_eq!("YWFh", r);
    }

    // Encode some data that results in a Base64 String with 1 padding character
    #[test]
    fn encode_basic_one_pad() {
        let data = "aa";
        let r = Base64Encoder::default().encode(data.as_bytes());
        assert_eq!("YWE=", r);
    }

    // Encode some data that results in a Base64 String with 2 padding characters
    #[test]
    fn encode_basic_two_pad() {
        let data = "a";
        let r = Base64Encoder::default().encode(data.as_bytes());
        assert_eq!("YQ==", r);
    }

    // Decode a Base64 String with 0 padding characters
    #[test]
    fn decode_basic_zero_pad() {
        let data = "aaa";
        let encoded = Base64Encoder::default().encode(data.as_bytes());

        match Base64Encoder::default().decode(&encoded) {
            Ok(v) => assert_eq!(data.as_bytes(), v),
            Err(_) => assert!(false),
        };
    }

    // Decode a Base64 String with 1 padding character
    #[test]
    fn decode_basic_one_pad() {
        let data = "aa";
        let encoded = Base64Encoder::default().encode(data.as_bytes());

        match Base64Encoder::default().decode(&encoded) {
            Ok(v) => assert_eq!(data.as_bytes(), v),
            Err(_) => assert!(false),
        };
    }

    // Decode a Base64 String with 2 padding characters
    #[test]
    fn decode_basic_two_pad() {
        let data = "a";
        let encoded = Base64Encoder::default().encode(data.as_bytes());

        match Base64Encoder::default().decode(&encoded) {
            Ok(v) => assert_eq!(data.as_bytes(), v),
            Err(_) => assert!(false),
        };
    }

    // Attempt to decode a string with invalid input length
    #[test]
    fn decode_basic_invalid_length() {
        let data = "a";

        match Base64Encoder::default().decode(&data) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                Base64Error::InvalidFormat(_) => assert!(false),
                Base64Error::InvalidInputLength(s) => assert_eq!(s, 1),
            },
        }
    }

    // Attempt to decode a string with invalid formatting, but valid length
    #[test]
    fn decode_basic_invalid_fmt() {
        let data = "A=AA==AA";

        match Base64Encoder::default().decode(&data) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                Base64Error::InvalidInputLength(_) => assert!(false),
                Base64Error::InvalidFormat(_) => assert!(true),
            },
        }
    }
}
