pub fn decode(s: &str) -> Vec<u8> {
    use himitsu::encode::hex::HexDecoder;
    s.chars().decode_hex()
}

pub fn decode_into_array<const B: usize>(s: &str) -> [u8; B] {
    decode(s).try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!("Expected a Vec of length {} but it was {}", B, v.len())
    })
}

pub fn decode_into_blocks<const B: usize>(s: &str) -> Vec<[u8; B]> {

    let mut blocks = Vec::new();

    let as_vec = decode(s);
    let chunks = as_vec.chunks(B);

    for chunk in chunks {

        let mut block = [0; B];
        for (i, byte) in chunk.iter().enumerate() {
            block[i] = *byte;
        }

        blocks.push(block);
    }

    blocks
}
