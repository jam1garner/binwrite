use std::io::{Write, Seek, Result, SeekFrom};
use std::ops::{Deref, DerefMut};

pub struct WriteTrack<W: Write> {
    inner: W,
    pos: usize,
}

impl<W: Write> WriteTrack<W> {
    pub fn new(inner: W) -> Self {
        WriteTrack {
            inner, pos: 0
        }
    }
}

impl<W: Write> Deref for WriteTrack<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.inner
    }
}

impl<W: Write> DerefMut for WriteTrack<W> {
    fn deref_mut(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W: Write> Write for WriteTrack<W> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        let amount = self.inner.write(data)?;
        self.pos += amount;
        Ok(amount)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

impl<W: Write> Seek for WriteTrack<W> {
    fn seek(&mut self, from: SeekFrom) -> Result<u64> {
        match from {
            SeekFrom::Current(0) | SeekFrom::End(0) => {
                Ok(self.pos as u64)
            }
            _ => {
                Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
            }
        }
    }
}
