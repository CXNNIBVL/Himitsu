use thiserror::Error as ThisErr;

#[derive(ThisErr, Debug)]
pub enum Base64Error {
    #[error("input length must be a multiple of 4 (found {0})")]
    InvalidInputLength(usize),

    #[error("invalid length after stripping non-base64 characters, remainder must be either 0, 2 or 3 (found {0})")]
    InvalidFormat(usize),
}