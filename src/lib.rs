#![allow(dead_code)]

#[cfg(feature = "d00b")]
mod d00b;
#[cfg(feature = "d95b")]
mod d95b;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub msg: String,
}
