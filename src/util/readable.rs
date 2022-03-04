use std::io;

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

impl<T: IntoIterator<Item = u8>> io::Read for Readable<T> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        let mut read = 0;

        for b in buf {
            if let Some(v) = self.it.next() {
                *b = v;
                read += 1;
            } else { break; }
        }

        Ok(read)
    }
}

pub struct XofReadable<T> 
    where T: IntoIterator<Item = u8>
{
    it: T::IntoIter,
}

impl<T: IntoIterator<Item = u8>> XofReadable<T> {
    pub fn new(item: T) -> Self {
        Self { 
            it: item.into_iter(),
        }
    }
} 

impl<T: IntoIterator<Item = u8>> io::Read for XofReadable<T> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        let mut read = 0;

        for b in buf {
            if let Some(v) = self.it.next() {
                *b = v;
                read += 1;
            } else { break; }
        }

        Ok(read)
    }

    fn read_to_end(&mut self, _buf: &mut Vec<u8>) -> io::Result<usize> {
        unimplemented!()
    }
    
}

#[cfg(test)]
mod tests {

    use std::io::Read;

    use super::*;

    #[test]
    fn test_readable() {

        let data = vec![1 , 2, 3, 4];
        let mut rdb: Readable<Vec<u8>> = Readable::new(data.clone());
        let mut out = Vec::new();

        rdb.read_to_end(&mut out).unwrap();

        assert_eq!(data, out);
    }
}

