#![allow(clippy::needless_return)]
#![feature(box_syntax)]
#![feature(map_first_last)]
#![feature(arbitrary_enum_discriminant)]

mod command;
mod errors;
pub mod nodes;
mod store;
pub mod traits;

pub use self::{
    errors::{NoteError, NoteErrorKind, Result},
    nodes::{ASTKind, ASTNode, ASTNodes},
    store::*,
};

pub mod utils {
    pub use text_utils::*;
    pub use yggdrasil_shared::records::*;
}
