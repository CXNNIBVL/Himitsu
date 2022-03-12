
pub fn decode(s: &str) -> Vec<u8> {
    use himitsu::encode::HexEncoder;
    HexEncoder::builder().decode(s)
}