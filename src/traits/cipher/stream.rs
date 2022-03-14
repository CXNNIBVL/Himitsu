use std::io;
use crate::util::readable::Readable;

pub trait StreamCipherEncryption: io::Write {
    type Output: IntoIterator<Item = u8>;
    fn finalize(self) -> Readable<Self::Output>;
}

pub trait StreamCipherDecryption: io::Write {
    type Output: IntoIterator<Item = u8>;
    fn finalize(self) -> Readable<Self::Output>;
}