use *;
use byteorder::{LE, BE};

/// A reader that will store whether to read in little or big endian
pub enum AutoEndianReader<R> {
    /// Little endian reader
    Little(R),
    /// Big endian reader
    Big(R)
}

/// An iterator over `char`s from an `AutoEndianReader`
pub enum AutoEndianChars<R> {
    /// Little endian reader
    Little(Chars<LE, R>),
    /// Big endian reader
    Big(Chars<BE, R>)
}

/// An iterator over `u16`s from an `AutoEndianReader`
pub enum AutoEndianShorts<R> {
    /// Little endian reader
    Little(Shorts<LE, R>),
    /// Big endian reader
    Big(Shorts<BE, R>)
}

/// An iterator over lines from an `AutoEndianReader`
pub enum AutoEndianLines<R> {
    /// Little endian reader
    Little(Lines<LE, R>),
    /// Big endian reader
    Big(Lines<BE, R>)
}

impl<R> AutoEndianReader<R> {
    /// Makes a new `AutoEndianReader` in little endian
    pub fn new_little(inner: R) -> Self {
        AutoEndianReader::Little(inner)
    }
    /// Makes a new `AutoEndianReader` in big endian
    pub fn new_big(inner: R) -> Self {
        AutoEndianReader::Big(inner)
    }
    /// Returns true if this reader is little endian
    pub fn is_little(&self) -> bool {
        match *self {
            AutoEndianReader::Little(_) => true,
            _ => false,
        }
    }
    /// Returns true if this reader is big endian
    pub fn is_big(&self) -> bool {
        match *self {
            AutoEndianReader::Big(_) => true,
            _ => false,
        }
    }
}

impl<R: Utf16ReadExt> AutoEndianReader<R> {
    /// Reads a `u16` to detect the endianness
    ///
    /// If the value isn't a valid bom (U+FEFF), an error is thrown
    pub fn new_auto_bom(mut inner: R) -> Result<Self, Error> {
        let bom = inner.read_u16::<LE>()?;
        match bom {
            0xfeff => Ok(AutoEndianReader::Little(inner)),
            0xfffe => Ok(AutoEndianReader::Big(inner)),
            _ => Err(Error::new(ErrorKind::InvalidData, "First character wasn't a bom"))
        }
    }
    /// Mirror of `Utf16ReadExt::read_u16` without the type parameter for endianness
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        match *self {
            AutoEndianReader::Little(ref mut r) => r.read_u16::<LE>(),
            AutoEndianReader::Big(ref mut r) => r.read_u16::<BE>(),
        }
    }
    /// Mirror of `Utf16ReadExt::shorts` without the type parameter for endianness
    pub fn shorts(self) -> AutoEndianShorts<R>
    where Self: Sized {
        match self {
            AutoEndianReader::Little(r) => AutoEndianShorts::Little(r.shorts::<LE>()),
            AutoEndianReader::Big(r) => AutoEndianShorts::Big(r.shorts::<BE>()),
        }
    }
    /// Mirror of `Utf16ReadExt::utf16_chars` without the type parameter for endianness
    pub fn utf16_chars(self) -> AutoEndianChars<R>
    where Self: Sized {
        match self {
            AutoEndianReader::Little(r) => AutoEndianChars::Little(r.utf16_chars()),
            AutoEndianReader::Big(r) => AutoEndianChars::Big(r.utf16_chars()),
        }
    }
    /// Mirror of `Utf16ReadExt::read_utf16_line` without the type parameter for endianness
    pub fn read_utf16_line(&mut self, buf: &mut String) -> Result<usize, Error> {
        match *self {
            AutoEndianReader::Little(ref mut r) => r.read_utf16_line::<LE>(buf),
            AutoEndianReader::Big(ref mut r) => r.read_utf16_line::<BE>(buf),
        }
    }
    /// Mirror of `Utf16ReadExt::utf16_lines` without the type parameter for endianness
    pub fn utf16_lines(self) -> AutoEndianLines<R>
    where Self: Sized {
        match self {
            AutoEndianReader::Little(r) => AutoEndianLines::Little(r.utf16_lines()),
            AutoEndianReader::Big(r) => AutoEndianLines::Big(r.utf16_lines()),
        }
    }
}

impl<R: Utf16ReadExt> Iterator for AutoEndianChars<R> {
    type Item = Result<char, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            AutoEndianChars::Little(ref mut r) => r.next(),
            AutoEndianChars::Big(ref mut r) => r.next(),
        }
    }
}

impl<R: Utf16ReadExt> Iterator for AutoEndianShorts<R> {
    type Item = Result<u16, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            AutoEndianShorts::Little(ref mut r) => r.next(),
            AutoEndianShorts::Big(ref mut r) => r.next(),
        }
    }
}

impl<R: Utf16ReadExt> Iterator for AutoEndianLines<R> {
    type Item = Result<String, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            AutoEndianLines::Little(ref mut r) => r.next(),
            AutoEndianLines::Big(ref mut r) => r.next(),
        }
    }
}
