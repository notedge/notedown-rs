use super::*;

impl From<std::io::Error> for NoteError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: Box::new(NoteErrorKind::IOError(e)), file: None, range: (0, 0) }
    }
}

impl From<std::fmt::Error> for NoteError {
    fn from(e: std::fmt::Error) -> Self {
        Self { kind: Box::new(NoteErrorKind::FormatError(e)), file: None, range: (0, 0) }
    }
}

