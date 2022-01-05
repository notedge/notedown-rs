#![forbid(missing_docs)]
#![doc = include_str!("../Readme.md")]
#![allow(clippy::needless_return)]

mod error;
mod error_3rd;

pub use error::{DiagnosticLevel, MaybeRanged, NoteError, NoteErrorKind, Result};

#[cfg(feature = "git2")]
pub extern crate git2;
#[cfg(feature = "globset")]
pub extern crate globset;
#[cfg(feature = "num")]
pub extern crate num;
