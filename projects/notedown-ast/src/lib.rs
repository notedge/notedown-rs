#![allow(clippy::needless_return)]

mod command;
mod errors;
pub mod nodes;
pub mod traits;

pub use errors::{NoteError, Result};
pub use nodes::{ASTKind, ASTNode, ASTNodes};

pub mod utils {
    pub use yggdrasil_shared::records::*;
    pub use text_utils::*;
}