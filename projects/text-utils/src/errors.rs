use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// Result of text progress
pub type Result<T> = std::result::Result<T, TextError>;

/// Error of text progress
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextError {
    UnescapeError { offset: usize, chars: String },
}

impl TextError {
    /// Constructor of [`TextError::UnescapeError`]
    pub fn unescape_error<T>(offset: usize, msg: impl Into<String>) -> Result<T> {
        Err(Self::UnescapeError { offset, chars: msg.into() })
    }
}

impl Error for TextError {}

impl Display for TextError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TextError::UnescapeError { offset, chars: message } => {
                write!(f, "UnescapeError: Fail to unescape `{}` at position {}", message, offset)
            }
        }
    }
}
