use thiserror::Error as ThisErr;

#[derive(ThisErr, Debug)]
pub enum Error {
    #[error("input length must be a multiple of 4 (found {0})")]
    InvalidInputLength(usize),

    #[error("invalid length after stripping non-base64 characters, remainder must be either 0, 2 or 3 (found {0})")]
    InvalidFormat(usize)
}

const B64_CHARS: [char; 64] = [
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
'+', '/'
];

const PADDING: char = '=';

/// Encodes bytes to a String in Base64 format
/// * 'bytes' - The byte buffer to encode
pub fn base64_encode(bytes: &[u8]) -> String {

    let mut encoded = String::new();

    if bytes.is_empty() {
        return encoded;
    }

    // Bytes are split into chunks of 6 bit each -> Must add up to multiple of 24 bit 
    let mut chunks = bytes.chunks_exact(3);

    while let Some(ch) = chunks.next() {
        // Main encoding step      
        let ia = ch[0] >> 2;
        let ib = ( ( ch[0] & 0b11 ) << 4) | ( ( ch[1] & 0b11110000 ) >> 4 );
        let ic = ( ( ch[1] & 0b1111 ) << 2) | ( ( ch[2] & 0b11000000 ) >> 6 );
        let id = ch[2] & 0b111111;

        encoded.extend([
            B64_CHARS[ia as usize], 
            B64_CHARS[ib as usize], 
            B64_CHARS[ic as usize], 
            B64_CHARS[id as usize]
        ]);
    }

    let rem = chunks.remainder().to_owned();

    // Each PADDING character amounts to two zero bits that have been appended to the remaining bits
    if rem.len() == 1 {

        let ia = rem[0] >> 2;
        let ib = (rem[0] & 0b11 ) << 4;

        encoded.extend([
            B64_CHARS[ia as usize],
            B64_CHARS[ib as usize],
            PADDING,
            PADDING
        ]);

    } else if rem.len() == 2 {

        let ia = rem[0] >> 2;
        let ib = ( ( rem[0] & 0b11 ) << 4) | ( ( rem[1] & 0b11110000 ) >> 4 );
        let ic = ( rem[1] & 0b1111 ) << 2;

        encoded.extend([
            B64_CHARS[ia as usize],
            B64_CHARS[ib as usize],
            B64_CHARS[ic as usize],
            PADDING
        ]);
    }

    encoded
}

// Returns the indices into the encoding array
fn is_b64(c: char) -> Option<u8> { 
    match c {
        'A'..='Z' => Some(c as u8 - b'A'),
        'a'..='z' => Some(c as u8 - b'a' + 26),
        '0'..='9' => Some(c as u8 - b'0' + 52),
        '+' => Some(62),
        '/' => Some(63),
        _ => None
    }
}

/// Decodes a String in Base64 format to bytes
/// 
/// Note: Will filter out any non-base64 characters
/// * 'string' - The string to decode
pub fn base64_decode(string: &str) -> Result<Vec<u8>, Error> {

    if string.len() % 4 != 0 { return Err(Error::InvalidInputLength(string.len())); }

    let mut decoded = Vec::new();

    // filter out any non-b64 chars
    let filtered: Vec<u8>  = string.chars()
                                .filter_map(|c| is_b64(c))
                                .collect();

    let mut chunks = filtered.chunks_exact(4);

    while let Some(ch) = chunks.next() {
        decoded.push( ( ch[0] << 2 ) | ( ch[1] >> 4 ) );
        decoded.push( ( ch[1] << 4 ) | ( ch[2] >> 2) );
        decoded.push( ( ch[2] << 6 ) | ch[3] );
    }

    let rem = chunks.remainder().to_owned();

    if rem.len() == 2 {

        decoded.push( ( rem[0] << 2 ) | ( rem[1] >> 4 ) )

    } else if rem.len() == 3 {

        decoded.push( ( rem[0] << 2 ) | ( rem[1] >> 4) );
        decoded.push( (rem[1] << 4) | (rem[2] >> 2) );

    } else if rem.len() > 3 || ( rem.len() < 2 && !rem.is_empty() ) { return Err(Error::InvalidFormat(rem.len())); }

    Ok(decoded)
}



#[cfg(test)]
mod tests {

    use super::*;

    // Encode some data that results in a Base64 String with 0 padding characters
    #[test]
    fn encode_zero_pad() {
        let data = "aaa";
        let r = base64_encode(data.as_bytes());
        assert_eq!("YWFh", r);
    }

    // Encode some data that results in a Base64 String with 1 padding character
    #[test]
    fn encode_one_pad() {
        let data = "aa";
        let r = base64_encode(data.as_bytes());
        assert_eq!("YWE=", r);
    }

    // Encode some data that results in a Base64 String with 2 padding characters
    #[test]
    fn encode_two_pad() {
        let data = "a";
        let r = base64_encode(data.as_bytes());
        assert_eq!("YQ==", r);
    }

    // Decode a Base64 String with 0 padding characters
    #[test]
    fn decode_zero_pad() {
        let data = "aaa";
        let encoded = base64_encode(data.as_bytes());
        
        match base64_decode(&encoded) {
            Ok(v) => assert_eq!(data.as_bytes(), v),
            Err(_) => assert!(false)
        };
    }

    // Decode a Base64 String with 1 padding character
    #[test]
    fn decode_one_pad() {
        let data = "aa";
        let encoded = base64_encode(data.as_bytes());

        match base64_decode(&encoded) {
            Ok(v) => assert_eq!(data.as_bytes(), v),
            Err(_) => assert!(false)
        };
    }

    // Decode a Base64 String with 2 padding characters
    #[test]
    fn decode_two_pad() {
        let data = "a";
        let encoded = base64_encode(data.as_bytes());

        match base64_decode(&encoded) {
            Ok(v) => assert_eq!(data.as_bytes(), v),
            Err(_) => assert!(false)
        };
    }

    // Attempt to decode a string with invalid input length
    #[test]
    fn decode_invalid_length() {
        let data = "a";
        
        match base64_decode(&data) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                Error::InvalidFormat(_) => assert!(false),
                Error::InvalidInputLength(s) => assert_eq!(s, 1)
            }
        }
    }

    // Attempt to decode a string with invalid formatting, but valid length
    #[test]
    fn decode_invalid_fmt() {
        let data = "A=AA==AA";

        match base64_decode(&data) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                Error::InvalidInputLength(_) => assert!(false),
                Error::InvalidFormat(_) => assert!(true)
            }
        }
    }
}