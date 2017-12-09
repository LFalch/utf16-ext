use std::io::{Write, Result, Error, ErrorKind};

use byteorder::{ByteOrder, WriteBytesExt};

/// An extension of `std::io::Write` for utf16
pub trait Utf16WriteExt: WriteBytesExt {
    /// Like `Write::write` but with `u16`s
    fn write_shorts<T: ByteOrder>(&mut self, buf: &[u16]) -> Result<usize> {
        let mut len = 0;
        for &short in buf {
            match self.write_u16::<T>(short) {
                Ok(()) => (),
                Err(_) if len > 0 => return Ok(len),
                Err(e) => return Err(e),
            }
            len += 1;
        }
        Ok(len)
    }
    /// Like `Write::write_all` but with `u16`s
    fn write_all_shorts<T: ByteOrder>(&mut self, mut buf: &[u16]) -> Result<()> {
        while !buf.is_empty() {
            match self.write_shorts::<T>(buf) {
                Ok(0) => return Err(Error::new(ErrorKind::WriteZero,
                                               "failed to write whole buffer")),
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    /// Writes a byte order maker character
    fn write_bom<T: ByteOrder>(&mut self) -> Result<()> {
        self.write_u16::<T>(0xfeff)
    }
    /// Writes a string as UTF-16
    ///
    /// Returns Ok(len) of the string written so far
    fn write_utf16_string<'a, T: ByteOrder>(&mut self, s: &'a str) -> Result<Utf16Written<'a>> {
        let mut encoder = s.encode_utf16();

        if let Some(short) = encoder.next() {
            match self.write_u16::<T>(short) {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
        }
        while let Some(short) = encoder.next() {
            match self.write_u16::<T>(short) {
                Ok(()) => (),
                Err(_) => return Ok(Utf16Written::Missing(encoder)),
            }
        }
        Ok(Utf16Written::FullyComplete)
    }
}

impl<T: Write> Utf16WriteExt for T {}

use std::str::EncodeUtf16;

/// Represents how much a string buffer was written
pub enum Utf16Written<'a> {
    /// Indicates that the whole string buffer written without errors
    FullyComplete,
    /// Indicates an erorr occured when writing, also gives the rest of the encoder
    Missing(EncodeUtf16<'a>)
}
