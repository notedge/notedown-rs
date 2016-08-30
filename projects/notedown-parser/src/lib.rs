mod config;
mod parser;
pub mod utils;
// pub mod utils;
pub use config::NotedownParser;
pub use notedown_ast::{NoteError, Result};

pub use notedown_ast::{
    nodes::{CommandKind, SmartLink},
    ASTKind, ASTNode,
};
