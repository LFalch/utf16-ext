#![warn(missing_docs)]
//! Crate for extending the `Read` and `Write` traits to allow
//! for reading and writing utf-16
pub extern crate byteorder;

mod auto;
mod read;
mod write;

pub use auto::*;
pub use read::*;
pub use write::*;
