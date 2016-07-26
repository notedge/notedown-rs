use lsp_types::Url;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use thiserror::Error;

mod error_custom;

pub type Result<T> = std::result::Result<T, NoteError>;

#[derive(Clone, Debug)]
pub struct NoteError {
    kind: Box<NoteErrorKind>,
    file: Option<Url>,
    range: (u32, u32),
}

#[derive(Clone, Debug)]
pub enum NoteErrorKind {
    IOError {},
    FormatError {},
    // PestError { #[from] source: pest::error::Error<crate::cst::Rule> },
    LanguageError {
        error: String,
    },
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
    TypeMismatch(String),
    RuntimeError(String),
    InfoMissing {
        error: String,
    },
    /// Some nodes failed to resolve and are being rolled back
    Unwinding,
    /// A forbidden cst_node encountered
    Unreachable,
    /* #[error(transparent)]
     * UnknownError(#[from] anyhow::Error), */
}

impl NoteError {
    pub fn set_url(self, url: Url) -> Self {
        Self { kind: self.kind, file: Some(url), range: self.range }
    }
    pub fn set_range(self, range: (u32, u32)) -> Self {
        Self { kind: self.kind, file: self.file, range }
    }

    // pub fn structure_error(msg: impl Into<String>, start: Option<usize>, end: Option<usize>) -> NoteError {
    //     Self::StructureError { error: msg.into(), start, end }
    // }
    // ///
    // pub fn unexpected_token(msg: impl Into<String>, start: Option<usize>, end: Option<usize>) -> NoteError {
    //     Self::UnexpectedToken { error: msg.into(), start, end }
    // }
    // ///
    // pub fn language_error(msg: impl Into<String>) -> NoteError {
    //     Self::LanguageError { error: msg.into() }
    // }
    ///
    pub fn type_mismatch(msg: impl Into<String>) -> NoteError {
        let kind = NoteErrorKind::TypeMismatch(msg.into());
        Self { kind: Box::new(kind), file: None, range: (0, 0) }
    }
    ///
    pub fn runtime_error(msg: impl Into<String>) -> NoteError {
        let kind = NoteErrorKind::RuntimeError(msg.into());
        Self { kind: Box::new(kind), file: None, range: (0, 0) }
    }
}

impl Display for NoteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for NoteError {}
