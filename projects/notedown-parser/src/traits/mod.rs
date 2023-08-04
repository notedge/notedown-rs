use notedown_ast::{NotedownAST, NotedownTerm};
use notedown_error::{NoteError, ParseResult, ParseState, Url};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    path::Path,
};

pub trait NoteParser
where
    Self: Sized,
{
    fn parse(input: ParseState) -> ParseResult<Self>;
    fn parse_text(input: &str) -> Result<Self, NoteError> {
        let input = ParseState::new(input);
        let (state, repl) = match Self::parse(input) {
            ParseResult::Pending(s, v) => (s, v),
            ParseResult::Stop(e) => Err(NoteError::custom(format!("Failed to parse text: {:?}", e)))?,
        };
        if !state.residual.is_empty() {
            Err(NoteError::custom(format!("Expect EOF, found:\n{}", state.residual)))?
        }
        Ok(repl)
    }
}

impl NoteParser for NotedownAST {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input.match_repeats(NotedownTerm::parse)?;
        state.finish(Self { terms, path: None })
    }
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<NotedownAST, NoteError> {
    let url = Url::from_file_path(path.as_ref())?;
    let text = std::fs::read_to_string(path)?;
    let (_, mut out) = NotedownAST::parse(ParseState::new(&text)).as_result()?;
    out.path = Some(url);
    Ok(out)
}
