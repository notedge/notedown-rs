#![allow(clippy::needless_return)]

extern crate text_utils;
mod command;
mod errors;
pub mod nodes;
pub mod traits;

pub use errors::{NoteError, Result};
pub use nodes::{ASTKind, ASTNode, ASTNodes};
