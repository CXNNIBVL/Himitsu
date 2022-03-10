const CHARSET_UPPERCASE: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
const CHARSET_LOWERCASE: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

#[derive(Debug, Clone)]
pub struct HexEncoder {
    header: String,
    seperator: String,
    terminator: String,
    groupsize: usize,
    case: Case
}

impl HexEncoder {

    pub fn builder() -> Self {
        Self {
            header: String::from(""),
            seperator: String::from(""),
            terminator: String::from(""),
            groupsize: 0,
            case: Case::Upper
        }
    }

    /// Encode in uppercase letters
    pub fn set_uppercase<'a>(&'a mut self) -> &'a mut Self {
        self.case = Case::Upper;
        self
    }

    /// Encode in lowercase letters
    pub fn set_lowercase<'a>(&'a mut self) -> &'a mut Self {
        self.case = Case::Lower;
        self
    }

    /// Will be prepended to the resulting string
    pub fn with_header<'a>(&'a mut self, header: &str) -> &'a mut Self {
        self.header = String::from(header);
        self
    }

    /// Will be inserted after each encoded grouping
    pub fn with_seperator<'a>(&'a mut self, seperator: &str) -> &'a mut Self {
        self.seperator = String::from(seperator);
        self
    }

    /// Will be appended to the resulting string
    pub fn with_terminator<'a>(&'a mut self, terminator: &str) -> &'a mut Self {
        self.terminator = String::from(terminator);
        self
    }

    /// Controls the byte grouping -> eg groupsize = 1: 01:02; groupsize = 2: 0102
    pub fn set_groupsize<'a>(&'a mut self, groupsize: usize) -> &'a mut Self {
        self.groupsize = groupsize;
        self
    }

    /// Encodes a byte buffer to a Hex string
    /// * 'data'    - The data to encode
    pub fn encode(&self, data: &[u8]) -> String {

        let mut encoded = String::from("");

        for (i, v) in data.iter().enumerate() {
            
            // Insert the seperator after each grouping
            if i % self.groupsize == 0 && i != 0 {
                encoded.push_str(&self.seperator);
            }
            
            // Charset indices
            let ix = ((v & 0xF0) >> 4) as usize;
            let iy = (v & 0x0F) as usize;

            // Insert the actual hex chars
            encoded.push(self.case.value_at(ix));
            encoded.push(self.case.value_at(iy));
        }

        // Format the output
        format!("{}{}{}", self.header, encoded, self.terminator)
    }

    /// Decodes a hex string into its bytes
    /// 
    /// Note: Will filter out any non-hex characters
    /// * 'hex'                                 - The hex string to decode
    /// * 'header', 'seperator', 'terminator'   - can also be set by the builder and will be stripped from the string
    pub fn decode(&self, hex: &str) -> Vec<u8> {

        let mut decoded = Vec::new();
        // remove header, seperator and terminator
        let stripped = {
            let mut s = hex.replace(&self.header, "");
            s = s.replace(&self.seperator, "");
            s.replace(&self.terminator, "")
        };

        // filter out any non-hex chars
        let mut filtered = stripped.chars().filter_map(|c| is_hex(c));

        while let Some(v1) = filtered.next() {
            match filtered.next() {
                Some(v2) => decoded.push((v1 << 4) | v2),
                _ => decoded.push(v1 << 4)
            }
        }

        decoded
    }
}

impl Default for HexEncoder {
    /// Create a new encoder with
    /// * no header
    /// * : as seperator
    /// * no terminator
    /// * 1 as groupsize
    /// * uppercase letters
    fn default() -> Self {
        Self {
            header: String::from(""),
            seperator: String::from(":"),
            terminator: String::from(""),
            groupsize: 1usize,
            case: Case::Upper
        }
    }
}

/// Upper or Lowercase characters
#[derive(Debug, Clone)]
enum Case {
    Upper,
    Lower
}

impl Case {
    fn value_at(&self, ix: usize) -> char {
        match self {
            Self::Upper => CHARSET_UPPERCASE[ix],
            Self::Lower => CHARSET_LOWERCASE[ix]
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