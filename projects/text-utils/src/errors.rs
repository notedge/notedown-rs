use std::fmt::{self, Debug, Formatter};

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
