#![allow(clippy::needless_return)]
#![feature(box_syntax)]

mod command;
mod errors;
pub mod nodes;
pub mod traits;

pub use errors::{NoteError, Result};
pub use nodes::{ASTKind, ASTNode, ASTNodes};

pub mod utils {
    pub use text_utils::*;
    pub use yggdrasil_shared::records::*;
}
