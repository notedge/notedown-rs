pub use super::*;
use std::fmt::{Formatter, Write};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhitespaceNode {
    width: usize,
    start: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewlineNode {
    count: usize,
    start: u32,
}

impl Display for WhitespaceNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.width {
            f.write_char(' ')?
        }
        Ok(())
    }
}

impl Display for NewlineNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.count {
            f.write_char('\n')?
        }
        Ok(())
    }
}
