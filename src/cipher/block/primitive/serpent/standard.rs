use super::implementation::AlgorithmProvider;

pub struct AlgorithmStandard();

impl AlgorithmProvider for AlgorithmStandard {
    fn new() -> Self {
        AlgorithmStandard()
    }
}