extern crate percent_encoding;
extern crate textwrap;

mod utils;

use std::fmt::{self, Debug, Formatter};
pub use utils::*;

pub enum TextError {
    UnescapeError(usize, Box<str>),
}

impl Debug for TextError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TextError::UnescapeError(p, c) => write!(f, "UnescapeError: Fail to unescape {} at position {}", c, p),
        }
    }
}
