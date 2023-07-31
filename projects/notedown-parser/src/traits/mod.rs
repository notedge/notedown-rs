use std::fmt::{Debug, Formatter};
use std::ops::Range;
use pex::{ParseResult, ParseState};
use crate::helpers::ignore;

pub struct ValkyrieError {
    kind: Box<ValkyrieErrorKind>,
}

#[derive(Debug)]
pub enum ValkyrieErrorKind {
    Custom {
        message: String,
    }
}

impl Debug for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl ValkyrieError {
    pub fn custom<T: ToString>(message: T) -> Self {
        Self {
            kind: Box::new(ValkyrieErrorKind::Custom {
                message: message.to_string(),
            }),
        }
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