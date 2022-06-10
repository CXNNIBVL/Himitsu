pub fn decode(s: &str) -> Vec<u8> {
    use himitsu::encode::hex::HexDecoder;
    s.chars().decode_hex()
}

pub fn decode_into_array<const B: usize>(s: &str) -> [u8; B] {
    use std::convert::TryInto;
    decode(s).try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!("Expected a Vec of length {} but it was {}", B, v.len())
    })
}
