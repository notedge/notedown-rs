#![allow(clippy::needless_return)]
#![feature(box_syntax)]
#![feature(map_first_last)]
#![feature(arbitrary_enum_discriminant)]

pub mod command;
mod errors;
pub mod nodes;
#[cfg(feature = "storage")]
mod store;
pub mod traits;
#[deny(missing_docs)]
/// Value and value types of notedown
pub mod value;

pub use self::{
    command::Command,
    errors::{NoteError, NoteErrorKind, Result},
    nodes::{ASTKind, ASTNode, ASTNodes},
    value::Value,
};

#[cfg(feature = "storage")]
pub use self::store::*;

pub mod utils {
    pub use itertools;
    pub use text_utils;
    pub use yggdrasil_shared::records::*;
}
