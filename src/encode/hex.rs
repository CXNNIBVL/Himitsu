
const CHARSET_UPPERCASE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
const CHARSET_LOWERCASE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];


/// Hex Decodes a stream of chars into a stream of bytes
/// 
/// The Decoder will ignore any characters that are not valid hex.
/// Additionally, the decoder cannot parse a string with prefixes:
/// e.g "0xFF 0xEE" will be parsed incorrectly as "0F, FE E0"
/// since the 0 in 0x will be interpreted as a hex character
pub trait HexDecoder: Iterator<Item = char> {
    fn decode_hex(self) -> Vec<u8>;
}

impl<I> HexDecoder for I  where I: Iterator<Item = char> {

    fn decode_hex(self) -> Vec<u8> {

        let mut decoded = Vec::with_capacity(self.size_hint().0);

        let mut filtered = self.filter_map(is_hex);

        while let Some(v1) = filtered.next() {
            match filtered.next() {
                Some(v2) => decoded.push((v1 << 4) | v2),
                _ => decoded.push(v1 << 4),
            }
        }

        decoded
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

#[derive(Debug, Clone)]
enum CharCase {
    Upper,
    Lower
}

impl CharCase {
    fn value_at(&self, ix: usize) -> char {
        match self {
            Self::Upper => CHARSET_UPPERCASE[ix],
            Self::Lower => CHARSET_LOWERCASE[ix],
        }
    }
}

#[derive(Debug, Clone)]
pub struct EncodingCfgBuilder {
    header: String,
    seperator: String,
    terminator: String,
    case: CharCase,
    byte_grouping: usize
}

impl EncodingCfgBuilder {

    pub fn new() -> Self {
        Self { 
            header: String::from(""), 
            seperator: String::from(""), 
            terminator: String::from(""), 
            case: CharCase::Upper, 
            byte_grouping: 1
        }
    }

    /// Encode in uppercase letters
    pub fn uppercase<'a>(mut self) -> Self {
        self.case = CharCase::Upper;
        self
    }

    /// Encode in lowercase letters
    pub fn lowercase<'a>(mut self) -> Self {
        self.case = CharCase::Lower;
        self
    }

    /// Will be prepended to the resulting string
    pub fn with_header<'a>(mut self, header: &str) -> Self {
        self.header = String::from(header);
        self
    }

    /// Will be inserted after each encoded grouping
    pub fn with_seperator<'a>(mut self, seperator: &str) -> Self {
        self.seperator = String::from(seperator);
        self
    }

    /// Will be appended to the resulting string
    pub fn with_terminator<'a>(mut self, terminator: &str) -> Self {
        self.terminator = String::from(terminator);
        self
    }

    /// Controls the byte grouping
    /// eg a groupsize of 1 will result in "01:02",
    /// whereas a groupsize = 2 will result in "0102"
    pub fn byte_grouping<'a>(mut self, byte_grouping: usize) -> Self {
        self.byte_grouping = if byte_grouping == 0 { 1 } else { byte_grouping };  
        self
    }

    pub fn build(self) -> EncodingCfg {
        EncodingCfg {
            header: self.header,
            seperator: self.seperator,
            terminator: self.terminator,
            case: self.case,
            byte_grouping: self.byte_grouping
        }
    }

}

#[derive(Debug, Clone)]
pub struct EncodingCfg {
    header: String,
    seperator: String,
    terminator: String,
    case: CharCase,
    byte_grouping: usize
}

impl EncodingCfg {
    pub fn builder() -> EncodingCfgBuilder {
        EncodingCfgBuilder::new()
    }
}

impl Default for EncodingCfg {
    fn default() -> Self {
        Self { 
            header: String::new(), 
            seperator: String::new(), 
            terminator: String::new(), 
            case: CharCase::Upper, 
            byte_grouping: 1 
        }
    }
}

pub trait HexEncoder: Iterator<Item = u8> {
    fn encode_hex(self, cfg: &EncodingCfg) -> String;
    fn encode_default_hex(self) -> String;
}

impl<I> HexEncoder for I 
    where I : Iterator<Item = u8>
{
    fn encode_default_hex(self) -> String {
        let cfg = EncodingCfg::default();
        self.encode_hex(&cfg)
    }

    fn encode_hex(self, cfg: &EncodingCfg) -> String {

        let mut encoded = String::new();

        for (i, v) in self.enumerate() {
            // Insert the seperator after each grouping
            if i % cfg.byte_grouping == 0 && i != 0 {
                encoded.push_str(&cfg.seperator);
            }

            // Charset indices
            let ix = ((v & 0xF0) >> 4) as usize;
            let iy = (v & 0x0F) as usize;

            // Insert the actual hex chars
            encoded.push(cfg.case.value_at(ix));
            encoded.push(cfg.case.value_at(iy));
        }

        format!("{}{}{}", cfg.header, encoded, cfg.terminator)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    // Decode a hex string
    #[test]
    fn decode() {
        let hx_s = "01:02:03:04:05";
        let decoded: Vec<u8> = hx_s.chars().decode_hex();

        let exp = vec![1, 2, 3, 4, 5];
        assert_eq!(decoded, exp)
    }

    // Encode some data with a groupsize of 1
    #[test]
    fn encode_groupsize_1() {
        let v = vec![1u8, 2, 3, 4];
        
        let cfg = EncodingCfg::builder()
            .byte_grouping(1)
            .with_seperator(":")
            .build();

        let encoded: String = v.into_iter().encode_hex(&cfg);


        let exp = "01:02:03:04";
        assert_eq!(encoded, exp);
    }

    // Encode some data with a groupsize of 2
    #[test]
    fn encode_groupsize_2() {
        let v = vec![1u8, 2, 3, 4];

        let cfg = EncodingCfg::builder()
        .byte_grouping(2)
        .with_seperator(":")
        .build();

        let encoded: String = v.into_iter().encode_hex(&cfg);

        let exp = "0102:0304";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 3
    #[test]
    fn encode_groupsize_3() {
        let v = vec![1u8, 2, 3, 4];

        let cfg = EncodingCfg::builder()
        .byte_grouping(3)
        .with_seperator(":")
        .build();

        let encoded: String = v.into_iter().encode_hex(&cfg);

        let exp = "010203:04";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 4
    #[test]
    fn encode_groupsize_4() {
        let v = vec![1u8, 2, 3, 4];

        let cfg = EncodingCfg::builder()
        .byte_grouping(4)
        .with_seperator(":")
        .build();

        let encoded: String = v.into_iter().encode_hex(&cfg);

        let exp = "01020304";
        assert_eq!(encoded, exp)
    }

    // Encode some data with a groupsize of 5
    #[test]
    fn encode_groupsize_5() {
        let v = vec![1u8, 2, 3, 4];

        let cfg = EncodingCfg::builder()
        .byte_grouping(5)
        .with_seperator(":")
        .build();

        let encoded: String = v.into_iter().encode_hex(&cfg);

        let exp = "01020304";
        assert_eq!(encoded, exp)
    }
}