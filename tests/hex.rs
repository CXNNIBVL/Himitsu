#[cfg(test)]
mod tests {

    use himitsu::encode::HexEncoder;

    // Decode a hex string
    #[test]
    fn decode() {
        let hx_s = "0x01 0x02 0x03 0x04 0x05";
        let decoded = HexEncoder::builder()
            .with_header("0x")
            .with_seperator(" 0x")
            .decode(hx_s);

        let exp = vec![1, 2, 3, 4, 5];
        assert_eq!(decoded, exp)
    }

    // Encode some data with a groupsize of 1
    #[test]
    fn encode_groupsize_1() {
        let v = vec![1u8, 2, 3, 4];
        let encoded = HexEncoder::default().encode(&v);
        let exp = "01:02:03:04";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 2
    #[test]
    fn encode_groupsize_2() {
        let v = vec![1u8, 2, 3, 4];
        let encoded = HexEncoder::default().set_groupsize(2).encode(&v);
        let exp = "0102:0304";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 3
    #[test]
    fn encode_groupsize_3() {
        let v = vec![1u8, 2, 3, 4];
        let encoded = HexEncoder::default().set_groupsize(3).encode(&v);
        let exp = "010203:04";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 4
    #[test]
    fn encode_groupsize_4() {
        let v = vec![1u8, 2, 3, 4];
        let encoded = HexEncoder::default().set_groupsize(4).encode(&v);
        let exp = "01020304";
        assert_eq!(encoded, exp)
    }
}
