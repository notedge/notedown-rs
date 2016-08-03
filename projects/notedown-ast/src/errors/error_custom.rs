use super::*;

impl From<std::io::Error> for NoteError {
    fn from(e: std::io::Error) -> Self {
        Self {
            kind: Box::new(NoteErrorKind::IOError(e)),
            file: None,
            range: (0, 0)
        }
    }
}