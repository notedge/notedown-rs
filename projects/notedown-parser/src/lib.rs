mod config;
mod error;
mod parser;
pub mod utils;
// pub mod utils;
pub use config::ParserConfig;
pub use error::{Error, ParserResult};

pub use notedown_ast::{CommandKind, SmartLink, TextRange, AST};
pub use notedown_pest;
// pub use note_text::{NoteTextParser, Rule as NoteTextRule};

pub fn parse(s: &str) -> AST {
    let cfg = ParserConfig::default();
    cfg.parse(s).unwrap_or_default()
}
