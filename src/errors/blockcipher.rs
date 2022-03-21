use thiserror::Error as ThisErr;

#[derive(Debug, ThisErr)]
pub enum BlockCipherError {
    #[error("last block is incomplete, found {0} missing bytes")]
    IncompleteBlock(usize),
}
