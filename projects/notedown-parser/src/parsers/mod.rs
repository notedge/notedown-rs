//! Parser combinators for Notedown language

use crate::helpers::{NotedownNode, ParseState};
use notedown_ast::ast::*;
use notedown_error::NoteError;

pub mod code;
pub mod command;
pub mod delimiter;
pub mod header;
pub mod link;
pub mod list;
pub mod math;
pub mod paragraph;
pub mod quote;
pub mod styled;
pub mod table;
pub mod text;
pub mod value;

// Re-export parsers

impl NotedownNode for NotedownAST {
    type Target = Self;

    fn parse(state: &mut ParseState) -> Result<Self::Target, NoteError> {
        todo!()
    }
}

impl NotedownNode for NotedownTerm {
    type Target = Self;

    fn parse(state: &mut ParseState) -> Result<Self::Target, NoteError> {
        todo!()
    }
}
