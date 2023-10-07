#![allow(dead_code)]

pub mod util;

#[cfg(feature = "d00b")]
pub mod d00b;
#[cfg(feature = "d95b")]
pub mod d95b;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub msg: String,
}
