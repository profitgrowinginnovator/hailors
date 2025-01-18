use std::io::{Read, Seek, SeekFrom, Result};

/// A wrapper for seekable byte readers.
pub struct SeekableBytesReader<R: Read + Seek> {
    inner: R,
}

impl<R: Read + Seek> SeekableBytesReader<R> {
    /// Creates a new `SeekableBytesReader`.
    pub fn new(inner: R) -> Self {
        SeekableBytesReader { inner }
    }

    /// Reads a specified number of bytes starting from the current position.
    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; length];
        self.inner.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Seeks to the specified position in the byte stream.
    pub fn seek_to(&mut self, position: u64) -> Result<u64> {
        self.inner.seek(SeekFrom::Start(position))
    }

    /// Returns the current position in the byte stream.
    pub fn current_position(&mut self) -> Result<u64> {
        self.inner.seek(SeekFrom::Current(0))
    }

    /// Reads a single u8 from the current position.
    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        self.inner.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    /// Reads a single u32 from the current position.
    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buffer = [0u8; 4];
        self.inner.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    /// Reads a single u64 from the current position.
    pub fn read_u64(&mut self) -> Result<u64> {
        let mut buffer = [0u8; 8];
        self.inner.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_seekable_bytes_reader() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let cursor = Cursor::new(data.clone());
        let mut reader = SeekableBytesReader::new(cursor);

        assert_eq!(reader.read_u8().unwrap(), 1);
        assert_eq!(reader.read_u32().unwrap(), 0x05040302);
        reader.seek_to(4).unwrap();
        assert_eq!(reader.read_u64().unwrap(), 0x0807060504030201);
    }
}
