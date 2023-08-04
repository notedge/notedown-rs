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
    Custom { message: String },
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

impl From<()> for NoteError {
    fn from(_: ()) -> Self {
        todo!()
    }
}

impl NoteError {
    pub fn custom<T: ToString>(message: T) -> Self {
        Self { kind: Box::new(NoteErrorKind::Custom { message: message.to_string() }) }
    }
}
