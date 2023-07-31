use std::ops::Range;
use pex::{ParseResult, ParseState};


pub struct ValkyrieError {}

impl ValkyrieError {
    pub fn custom<T>(msg: T) -> Self {
        unimplemented!()
    }
}

pub(crate) trait ThisParser
    where
        Self: Sized,
{
    fn parse(input: ParseState) -> ParseResult<Self>;
    fn parse_text(input: &str) -> Result<Self, ValkyrieError> {
        let input = ParseState::new(input);
        let (state, repl) = match Self::parse(input.skip(ignore)) {
            ParseResult::Pending(s, v) => (s.skip(ignore), v),
            ParseResult::Stop(e) => Err(ValkyrieError::custom(format!("Failed to parse text: {:?}", e)))?,
        };
        if !state.residual.is_empty() {
            Err(ValkyrieError::custom(format!("Expect EOF, found:\n{}", state.residual)))?
        }
        Ok(repl)
    }
    #[track_caller]
    fn get_range(&self) -> Range<u32> {
        unreachable!()
    }
}