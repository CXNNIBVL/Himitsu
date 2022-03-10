use crate::errors::base64::Base64Error;

const B64_CHARS: [char; 64] = [
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
'+', '/'
];

const B64_URL_CHARS: [char; 64] = [
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
'-', '_'
];

const PADDING: char = '=';

#[derive(Debug, Clone, Copy)]
pub struct Base64Encoder {
    kind: Kind
}

impl Base64Encoder {

    /// Create a new encoder with basic encoding
    pub fn new() -> Self {
        Self { kind: Kind::Basic }
    } 

    /// Create a new encoder with url safe encoding
    pub fn new_url() -> Self {
        Self { kind: Kind::UrlSafe }
    }

    /// Encodes bytes to a String in Base64 format
    /// * 'bytes' - The byte buffer to encode
    pub fn encode(&self, bytes: &[u8]) -> String {

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
                self.kind.value_at(ia as usize),
                self.kind.value_at(ib as usize), 
                self.kind.value_at(ic as usize),
                self.kind.value_at(id as usize)
            ]);
        }

        // Each PADDING character amounts to two zero bits that have been appended to the remaining bits
        if chunks.remainder().len() == 1 {

            let ia = chunks.remainder()[0] >> 2;
            let ib = (chunks.remainder()[0] & 0b11 ) << 4;

            encoded.extend([
                self.kind.value_at(ia as usize),
                self.kind.value_at(ib as usize),
                PADDING,
                PADDING
            ]);

        } else if chunks.remainder().len() == 2 {

            let ia = chunks.remainder()[0] >> 2;
            let ib = ( ( chunks.remainder()[0] & 0b11 ) << 4) | ( ( chunks.remainder()[1] & 0b11110000 ) >> 4 );
            let ic = ( chunks.remainder()[1] & 0b1111 ) << 2;

            encoded.extend([
                self.kind.value_at(ia as usize),
                self.kind.value_at(ib as usize),
                self.kind.value_at(ic as usize),
                PADDING
            ]);
        }

        encoded
    }

    /// Decodes a String in Base64 format to bytes
    /// 
    /// Note: Will filter out any non-base64 characters
    /// * 'string' - The string to decode
    pub fn decode(&self, string: &str) -> Result<Vec<u8>, Base64Error> {

        if string.len() % 4 != 0 { return Err(Base64Error::InvalidInputLength(string.len())); }

        // filter out any non-b64 chars
        let filtered: Vec<u8>  = string.chars()
                                    .filter_map(|c| self.kind.is_b64(c))
                                    .collect();

        decode_core(filtered)
    }
}

impl Default for Base64Encoder {
    /// Create new encoder with basic encoding
    fn default() -> Self {
        Self { kind: Kind::Basic }
    }
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Basic,
    UrlSafe
}

impl Kind {

    // Returns the character at the given index
    fn value_at(&self, ix: usize) -> char {
        match self {
            Kind::Basic => B64_CHARS[ix],
            Kind::UrlSafe => B64_URL_CHARS[ix]
        }
    }

    // Returns the indices into the encoding array
    fn is_b64(&self, c: char) -> Option<u8> {
         
        match self {
            Kind::Basic => match c {
                'A'..='Z' => Some(c as u8 - b'A'),
                'a'..='z' => Some(c as u8 - b'a' + 26),
                '0'..='9' => Some(c as u8 - b'0' + 52),
                '+' => Some(62),
                '/' => Some(63),
                _ => None
            },

            Kind::UrlSafe => match c {
                'A'..='Z' => Some(c as u8 - b'A'),
                'a'..='z' => Some(c as u8 - b'a' + 26),
                '0'..='9' => Some(c as u8 - b'0' + 52),
                '-' => Some(62),
                '_' => Some(63),
                _ => None
            }
        }
    }
}

// Core decoding function, returns decoded bytes
fn decode_core(filtered: Vec<u8>) -> Result<Vec<u8>, Base64Error> {

    let mut decoded = Vec::new();

    let mut chunks = filtered.chunks_exact(4);

    while let Some(ch) = chunks.next() {
        decoded.push( ( ch[0] << 2 ) | ( ch[1] >> 4 ) );
        decoded.push( ( ch[1] << 4 ) | ( ch[2] >> 2) );
        decoded.push( ( ch[2] << 6 ) | ch[3] );
    }

    match chunks.remainder().len() {
        0 => {},

        2 => decoded.push( ( chunks.remainder()[0] << 2 ) | ( chunks.remainder()[1] >> 4 ) ),

        3 => {
            decoded.push( ( chunks.remainder()[0] << 2 ) | ( chunks.remainder()[1] >> 4) );
            decoded.push( (chunks.remainder()[1] << 4) | (chunks.remainder()[2] >> 2) );
        },

        _ => return Err(Base64Error::InvalidFormat(chunks.remainder().len()))
    }

    Ok(decoded)
}