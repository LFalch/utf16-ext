use std::io::{Read, Error, ErrorKind};

use byteorder::{ByteOrder, ReadBytesExt};

/// Extension to the `Read` trait
pub trait Utf16ReadExt: ReadBytesExt {
    /// Transforms this instance into an `Iterator` over its u16-units (shorts).
    ///
    /// The returned type implements `Iterator` where the `Item` is `Result<u16, R::Err>`.
    /// The yielded item is `Ok` if a short was successfully read and `Err` otherwise.
    /// EOF is mapped to returning `None` from this iterator.
    fn shorts<T: ByteOrder>(self) -> Shorts<T, Self>
    where Self: Sized {
        Shorts(PhantomData, self)
    }
    /// Transforms this instance into an `Iterator` over `char`s from utf-16.
    ///
    /// The returned type implements `Iterator` where the `Item` is `Result<char, R::Err>`.
    fn utf16_chars<T: ByteOrder>(self) -> Chars<T, Self>
    where Self: Sized {
        Chars(PhantomData, self)
    }
    /// Reads all chars (from utf16) until a newline is reached (U+000A) and
    /// appends them to the provided buffer.
    fn read_utf16_line<T: ByteOrder>(&mut self, buf: &mut String) -> Result<usize, Error> {
        let mut len = 0;
        for c in self.utf16_chars::<T>() {
            match c {
                Ok(c) => {
                    buf.push(c);
                    len += 1;
                    if c == '\n' {
                        break
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => return Err(e),
                }
            }
        }
        Ok(len)
    }
    /// Returns an iterator over the lines of this reader.
    ///
    /// Like the normal `BufRead::lines`, newlines characters aren't included
    fn utf16_lines<T: ByteOrder>(self) -> Lines<T, Self>
    where Self: Sized {
        Lines(PhantomData, self)
    }
}

impl<T: Read> Utf16ReadExt for T {}

use std::marker::PhantomData;

#[derive(Debug)]
/// An iterator over `u16` values of a reader.
pub struct Shorts<T: ByteOrder, R>(PhantomData<T>, R);
#[derive(Debug)]
/// An iterator over `char` values of a utf-16 reader.
pub struct Chars<T: ByteOrder, R>(PhantomData<T>, R);

impl<T: ByteOrder, R: Utf16ReadExt> Iterator for Shorts<T, R> {
    type Item = Result<u16, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.1.read_u16::<T>() {
                Ok(u) => break Some(Ok(u)),
                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => (),
                    ErrorKind::UnexpectedEof => break None,
                    _ => break Some(Err(e)),
                }
            }
        }
    }
}

use std::char::decode_utf16;

impl<T: ByteOrder, R: Utf16ReadExt> Iterator for Chars<T, R> {
    type Item = Result<char, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let first = match self.1.read_u16::<T>() {
            Ok(f) => f,
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => return None,
            Err(e) => return Some(Err(e))
        };
        match decode_utf16(Some(first)).next().unwrap() {
            Ok(c) => Some(Ok(c)),
            Err(_) => {
                let snd = match self.1.read_u16::<T>() {
                    Ok(f) => f,
                    Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => return None,
                    Err(e) => return Some(Err(e))
                };
                Some(decode_utf16(Some(first).into_iter().chain(Some(snd))).next().unwrap()
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e)))
            }
        }
    }
}

#[derive(Debug)]
/// An iterator over the lines of a reader (reading utf-16)
pub struct Lines<T: ByteOrder, B>(PhantomData<T>, B);

impl<T: ByteOrder, B: Utf16ReadExt> Iterator for Lines<T, B> {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.1.read_utf16_line::<T>(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with("\n") {
                    buf.pop();
                    if buf.ends_with("\r") {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e))
        }
    }
}
