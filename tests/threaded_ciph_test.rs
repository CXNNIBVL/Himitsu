#[allow(dead_code)]


#[cfg(test)]
mod tests {

    use std::io::{Write, Read};
    use himitsu::cipher::blockcipher::primitive::aes;
    use himitsu::traits::cipher::*;

    

    #[test]
    fn threading_test() {

        let key = vec![1, 2, 3, 4, 5];
        let mut ciph = aes::Aes::new(&key).with_threaded_ecb_encryption(4);

        let data = [0; aes::AES_BLOCKSIZE];
        ciph.write_all(&data).unwrap(); 

        let mut out = Vec::new();
        ciph.finalize().unwrap().read_to_end(&mut out).unwrap();
    }

}