use super::implementation::AlgorithmProvider;

pub struct AlgorithmBitsliced();

impl AlgorithmProvider for AlgorithmBitsliced {
    fn new() -> Self {
        AlgorithmBitsliced()
    }
}