use crate::nodes::MaybeRanged;
use std::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
    path::Path,
};
use yggdrasil_shared::records::Url;

mod error_3rd;
mod error_custom;
mod error_std;

pub type Result<T> = std::result::Result<T, NoteError>;

#[derive(Debug)]
pub struct NoteError {
    pub kind: Box<NoteErrorKind>,
    pub file: Option<Url>,
    pub range: MaybeRanged,
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
        Self { kind: Box::new(NoteErrorKind::Unreachable), file: None, range: None }
    }
}
macro_rules! error_msg {
    ($name:ident => $t:ident) => {
        pub fn $name(msg: impl Into<String>) -> NoteError {
            let kind = NoteErrorKind::$t(msg.into());
            Self { kind: Box::new(kind), file: None, range: None }
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
