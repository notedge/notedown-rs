use crate::NoteError;
use pest::error::Error;
use std::fmt::Debug;

impl<R> From<Error<R>> for NoteError
where
    R: Debug,
{
    fn from(e: Error<R>) -> Self {
        Self::runtime_error(format!("{:?}", e))
    }
}
