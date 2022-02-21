pub use std::io::{Read as IORead, Result as IOResult, Error as IOError, ErrorKind as IOErrorKind};

pub struct Readable<T> 
    where T: IntoIterator<Item = u8>
{
    it: T::IntoIter,
}

impl<T: IntoIterator<Item = u8>> Readable<T> {
    pub fn new(item: T) -> Self {
        Self { 
            it: item.into_iter(),
        }
    }
} 

impl<T: IntoIterator<Item = u8>> IORead for Readable<T> {

    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {

        let mut read = 0;

        for b in buf {
            match self.it.next() {
                Some(v) => {
                    *b = v;
                    read += 1;
                },
                None => return Err(IOError::from(IOErrorKind::UnexpectedEof))
            }
        }

        Ok(read)
    }
    
}

#[cfg(test)]
mod tests {

    use std::io::Read;

    use super::*;

    #[test]
    fn test_readable() {

        let data = vec![1 , 2, 3, 4];
        let mut rdb: Readable<Vec<u8>> = Readable::new(data);
        let mut buf = vec![5, 5, 5, 5];

        match rdb.read(&mut buf) {
            Ok(v) => assert_eq!(4, v),
            _ => assert!(false)
        }


        match rdb.read(&mut buf) {
            Ok(_) => assert!(false),
            _ => assert!(true)
        }
    }

}

