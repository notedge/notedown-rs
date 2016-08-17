use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
};
use yggdrasil_shared::records::Url;

mod error_std;

pub type Result<T> = std::result::Result<T, NoteError>;

#[derive(Debug)]
pub struct NoteError {
    pub kind: Box<NoteErrorKind>,
    pub file: Option<Url>,
    pub range: Option<Range<usize>>,
}

#[derive(Debug)]
pub enum NoteErrorKind {
    IOError(std::io::Error),
    FormatError(std::fmt::Error),
    TypeMismatch(String),
    RuntimeError(String),
    /// A forbidden cst_node encountered
    Unreachable,
    // #[error(transparent)]
    // UnknownError(#[from] anyhow::Error),
}

impl NoteError {
    pub fn set_url(mut self, url: Url) -> Self {
        self.file = Some(url);
        return self;
    }
    pub fn set_range(mut self, start: usize, end: usize) -> Self {
        self.range = Some(Range { start, end });
        return self;
    }
    pub fn unreachable() -> Self {
        Self { kind: Box::new(NoteErrorKind::Unreachable), file: None, range: None }
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
        Self { kind: Box::new(kind), file: None, range: None }
    }
    ///
    pub fn runtime_error(msg: impl Into<String>) -> NoteError {
        let kind = NoteErrorKind::RuntimeError(msg.into());
        Self { kind: Box::new(kind), file: None, range: None }
    }
}

impl Error for NoteError {}

impl Display for NoteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let path = match &self.file {
            Some(s) => s.path(),
            None => "<Anonymous>",
        };
        match &self.range {
            Some(s) => writeln!(f, "at ({}, {}) of {}", s.start, s.end, path)?,
            None => writeln!(f, "at {}", path)?,
        }
        write!(f, "{:indent$}{}", self.kind, indent = 4)
    }
}

impl Display for NoteErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError { .. } => {
                unimplemented!()
            }
            Self::FormatError { .. } => {
                unimplemented!()
            }
            Self::TypeMismatch(msg) => {
                f.write_str("TypeMismatch: ")?;
                f.write_str(msg)
            }
            Self::RuntimeError(_) => {
                unimplemented!()
            }
            Self::Unreachable => {
                f.write_str("InternalError: ")?;
                f.write_str("Entered unreachable code!")
            }
        }
    }
}
