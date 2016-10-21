mod error_3rd;
mod error_std;

use std::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
    path::Path,
};
use url::Url;

/// All result about notedown
pub type Result<T> = std::result::Result<T, NoteError>;

#[derive(Debug)]
pub struct NoteError {
    pub kind: Box<NoteErrorKind>,
    pub level: DiagnosticLevel,
    pub file: Option<Url>,
    pub range: Option<Range<usize>>,
}

#[derive(Debug)]
pub enum NoteErrorKind {
    IOError(std::io::Error),
    FormatError(std::fmt::Error),
    SyntaxError(String),
    TypeMismatch(String),
    RuntimeError(String),
    UndefinedVariable {
        name: String,
    },
    /// A forbidden cst_node encountered
    Unreachable,
    // #[error(transparent)]
    // UnknownError(#[from] anyhow::Error),
}

#[derive(Debug, Copy, Clone)]
pub enum DiagnosticLevel {
    None = 0,
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

impl NoteError {
    #[inline]
    pub fn set_path(&mut self, path: impl AsRef<Path>) {
        if let Ok(s) = Url::from_file_path(path) {
            self.file = Some(s)
        }
    }
    #[inline]
    pub fn set_url(&mut self, url: Url) {
        self.file = Some(url);
    }
    #[inline]
    pub fn set_range(&mut self, start: usize, end: usize) {
        self.range = Some(Range { start, end });
    }
    #[inline]
    pub fn unreachable() -> Self {
        Self { kind: Box::new(NoteErrorKind::Unreachable), level: DiagnosticLevel::None, file: None, range: None }
    }
}

impl NoteError {
    /// Deprecated or obsolete code.
    /// Clients are allowed to rendered diagnostics with this tag strike through.
    pub fn is_deprecated(&self) -> bool {
        false
    }
    /// Unused or unnecessary code.
    /// Clients are allowed to render diagnostics with this tag faded out instead of having an error squiggle.
    pub fn is_unnecessary(&self) -> bool {
        false
    }
}

macro_rules! error_msg {
    ($name:ident => $t:ident) => {
        pub fn $name(msg: impl Into<String>) -> NoteError {
            let kind = NoteErrorKind::$t(msg.into());
            Self { kind: Box::new(kind), level: DiagnosticLevel::None, file: None, range: None }
        }
    };
    ($($name:ident => $t:ident),+ $(,)?) => (
        impl NoteError { $(error_msg!($name=>$t);)+ }
    );
}

error_msg![
    syntax_error => SyntaxError,
    type_mismatch => TypeMismatch,
    runtime_error => RuntimeError,
];

impl NoteError {
    pub fn undefined_variable(name: impl Into<String>) -> NoteError {
        let kind = NoteErrorKind::UndefinedVariable { name: name.into() };
        Self { kind: Box::new(kind), level: DiagnosticLevel::None, file: None, range: None }
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
            Self::IOError(e) => {
                write!(f, "{}", e)
            }
            Self::FormatError(e) => {
                write!(f, "{}", e)
            }
            Self::SyntaxError(msg) => {
                f.write_str("SyntaxError: ")?;
                f.write_str(msg)
            }
            Self::TypeMismatch(msg) => {
                f.write_str("TypeError: ")?;
                f.write_str(msg)
            }
            Self::RuntimeError(msg) => {
                f.write_str("RuntimeError: ")?;
                f.write_str(msg)
            }
            Self::UndefinedVariable { name } => {
                write!(f, "RuntimeError: Variable {} not found in scope", name)
            }
            Self::Unreachable => {
                f.write_str("InternalError: ")?;
                f.write_str("Entered unreachable code!")
            }
        }
    }
}
