use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use lsp_types::Url;

mod error_custom;

pub type Result<T> = std::result::Result<T, NoteError>;

#[rustfmt::skip]
#[derive(Error, Debug)]
pub enum NoteError {
    IOError { #[from] source: std::io::Error },
    FormatError { #[from] source: std::fmt::Error },
    // PestError { #[from] source: pest::error::Error<crate::cst::Rule> },
    LanguageError { error: String },
    StructureError {
        error: String,
        start: Option<usize>,
        end: Option<usize>,
    },
    UnexpectedToken {
        error: String,
        start: Option<usize>,
        end: Option<usize>,
    },
    TypeMismatch {
        error: String,
        file: Option<Url>,
        range: (u32, u32),
    },
    RuntimeError {
        error: String,
        file: Option<Url>,
        range: (u32, u32),
    },

    InfoMissing { error: String },
    /// Some nodes failed to resolve and are being rolled back
    Unwinding,
    /// A forbidden cst_node encountered
    Unreachable,
    // #[error(transparent)]
    // UnknownError(#[from] anyhow::Error),
}


impl NoteError {
    pub fn set_url(self, url: Url) -> Self {
        match self {
            Self::TypeMismatch { error, file: _, range } => {
                Self::TypeMismatch {
                    error,
                    file: Some(url),
                    range,
                }
            }
            _ => self,
        }
    }
    pub fn set_range(self, range: (u32, u32)) -> Self {
        match self {
            Self::TypeMismatch { error, file, range: _ } => {
                Self::TypeMismatch {
                    error,
                    file,
                    range,
                }
            }
            _ => self,
        }
    }

    pub fn structure_error(msg: impl Into<String>, start: Option<usize>, end: Option<usize>) -> NoteError {
        Self::StructureError { error: msg.into(), start, end }
    }
    ///
    pub fn unexpected_token(msg: impl Into<String>, start: Option<usize>, end: Option<usize>) -> NoteError {
        Self::UnexpectedToken { error: msg.into(), start, end }
    }
    ///
    pub fn language_error(msg: impl Into<String>) -> NoteError {
        Self::LanguageError { error: msg.into() }
    }
    ///
    pub fn type_mismatch(msg: impl Into<String>) -> NoteError {
        Self::TypeMismatch {
            error: msg.into(),
            file: None,
            range: (0, 0),
        }
    }
}

impl Display for NoteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
