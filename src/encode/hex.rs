const CHARSET_UPPERCASE: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
const CHARSET_LOWERCASE: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

/// Upper or Lowercase characters
pub enum Case {
    Upper,
    Lower
}

impl Case {
    fn value(&self) -> &'static [char;16] {
        match self {
            Self::Upper => &CHARSET_UPPERCASE,
            Self::Lower => &CHARSET_LOWERCASE
        }
    }
}

fn is_hex(character: char) -> Option<u8> {
    match character {
        '0'..='9' => Some(character as u8 - b'0'),
        'A'..='F' => Some(character as u8 - b'A' + 10),
        'a'..='f' => Some(character as u8 - b'a' + 10),
        _ => None,
    }
}

/// Encodes a byte buffer to a Hex string
/// * 'bytes'       - The buffer to encode
/// * 'case'   - Whether to encode in uppercase or lowercase letters
/// * 'header'      - will be prepended to the string
/// * 'seperator'   - will be inserted after each grouping
/// * 'terminator'  - will be appended to the resulting string
/// * 'groupsize'   - Controls the grouping. E.g groupsize 2 and seperator ":" with a buffer of \[1,0,1] will result in "0100:01"
pub fn hex_encode(bytes: &[u8], case: Case, header: &str, seperator: &str, terminator: &str, groupsize: usize) -> String {
    let charset = case.value();

    let mut encoded = String::from(header);

    for (i, v) in bytes.iter().enumerate() {
        
        // Insert the seperator after each grouping
        if i % groupsize == 0 && i != 0 {
            encoded.push_str(seperator);
        }
        
        // Charset indices
        let ix = ((v & 0xF0) >> 4) as usize;
        let iy = (v & 0x0F) as usize;

        // Insert the actual hex chars
        encoded.push(charset[ix]);
        encoded.push(charset[iy]);
    }

    // Insert the terminator
    encoded += terminator;

    encoded
}

/// Decodes a hex string into its bytes
/// 
/// Note: Will filter out any non-hex characters
/// * 'hex'                                 - The hex string to decode
/// * 'header', 'seperator', 'terminator'   - will be stripped from the string
pub fn hex_decode(hex: &str, header: &str, seperator: &str, terminator: &str) -> Vec<u8> {

    let mut decoded = Vec::new();
    // remove header, seperator and terminator
    let mut s = hex.replace(header, "");
    s = s.replace(seperator, "");
    s = s.replace(terminator, "");

    // filter out any non-hex chars
    let mut it = s.chars().filter_map(|c| is_hex(c));

    while let Some(v1) = it.next() {
        match it.next() {
            Some(v2) => decoded.push((v1 << 4) | v2),
            _ => decoded.push(v1 << 4)
        }
    }

    decoded
}

#[cfg(test)]
mod tests {

    use super::*;

    // Decode a hex string
    #[test]
    fn decode() {

        let hx_s = "0x01 0x02 0x03 0x04 0x05";
        let decoded = hex_decode(hx_s, "0x", " 0x", "");
        let exp = vec![1, 2, 3, 4, 5];
        assert_eq!(decoded, exp)
    }

    // Encode some data with a groupsize of 1
    #[test]
    fn encode_groupsize_1() {
        let v = vec![1u8, 2, 3, 4];
        let encoded = hex_encode(&v, Case::Upper, "", ":", "", 1);
        let exp = "01:02:03:04";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 2
    #[test]
    fn encode_groupsize_2() {
        let v = vec![1u8, 2, 3, 4];
        let encoded = hex_encode(&v, Case::Upper, "", ":", "", 2);
        let exp = "0102:0304";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 3
    #[test]
    fn encode_groupsize_3() {
        let v = vec![1u8, 2, 3, 4];
        let encoded = hex_encode(&v, Case::Upper, "", ":", "", 3);
        let exp = "010203:04";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 4
    #[test]
    fn encode_groupsize_4() {
        let v = vec![1u8, 2, 3, 4];
        let encoded = hex_encode(&v, Case::Upper, "", ":", "", 4);
        let exp = "01020304";
        assert_eq!(encoded, exp)
    }
}