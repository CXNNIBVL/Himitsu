pub use crate::traits::{
    cipher::block::{
        BlockCipherEncryption, 
        BlockCipherDecryption
    },
    cipher::primitive::{
        BlockCipherPrimitiveEncryption, 
        BlockCipherPrimitiveDecryption
    },
    cipher::stream::{
        StreamCipherEncryption,
        StreamCipherDecryption
    }
};