use std::fmt::{self, Debug, Formatter};

/// Result of text progress
pub type Result<T> = std::result::Result<T, TextError>;

/// Error of text progress
pub enum TextError {
    /// UnescapeError
    UnescapeError(usize, Box<str>),
}

impl Debug for TextError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TextError::UnescapeError(p, c) => write!(f, "UnescapeError: Fail to unescape {} at position {}", c, p),
        }
    }
}
