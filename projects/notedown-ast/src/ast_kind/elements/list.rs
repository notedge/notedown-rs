use crate::ast_kind::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ListView<T> {
    QuoteList { style: String, body: Vec<T> },
    OrderedList { head: usize, body: Vec<T> },
    OrderlessList { body: Vec<T> },
    Details { summary: T, body: Vec<T> },
}

impl<T: Debug> Display for ListView<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::QuoteList { style, body } => {
                writeln!(f, "QuoteList")?;
                writeln!(f, "{:?}", style)?;
                writeln!(f, "{:?}", body)?;
            }
            Self::OrderedList { .. } => {
                writeln!(f, "OrderedList")?;
            }
            Self::OrderlessList { .. } => {
                writeln!(f, "OrderlessList")?;
            }
            Self::Details { .. } => {
                writeln!(f, "Details")?;
            }
        }
        Ok(())
    }
}
