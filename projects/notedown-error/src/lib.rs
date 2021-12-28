#![forbid(missing_docs)]
#![doc = include_str!("../Readme.md")]
#![allow(clippy::needless_return)]

mod error;
mod error_3rd;

pub use error::{DiagnosticLevel, MaybeRanged, NoteError, NoteErrorKind, Result};
