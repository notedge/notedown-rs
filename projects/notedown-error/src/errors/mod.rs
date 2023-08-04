mod display;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub type Validation<T> = validatus::Validation<T, NoteError>;

pub struct NoteError {
    kind: Box<NoteErrorKind>,
}

pub enum NoteErrorKind {
    IOError { message: String, source: std::io::Error },
    Unknown { message: String },
}

impl From<std::io::Error> for NoteError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: Box::new(NoteErrorKind::IOError { message: "".to_string(), source: e }) }
    }
}

impl From<std::fmt::Error> for NoteError {
    fn from(value: std::fmt::Error) -> Self {
        todo!()
    }
}

impl From<()> for NoteErrorKind {
    fn from(_: ()) -> Self {
        Self::Unknown { message: "".to_string() }
    }
}
