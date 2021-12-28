#![forbid(missing_docs)]
#![doc = include_str!("../Readme.md")]

mod error;
mod error_3rd;

pub use error::{MaybeRanged, NoteError, NoteErrorKind, Result};
