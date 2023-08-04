use crate::NoteParser;
use notedown_ast::{NotedownAST, NotedownTerm};
use pex::{ParseResult, ParseState};
use std::path::Path;

impl NoteParser for NotedownAST {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input.match_repeats(NotedownTerm::parse)?;
        state.finish(Self { terms, path: None })
    }
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<NotedownAST, NoteError> {
    let mut path =

    let input = ParseState::new(&std::fs::read_to_string(path)?);
    NotedownAST::parse(input)
}
