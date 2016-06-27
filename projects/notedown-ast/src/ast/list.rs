use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ListView {
    QuoteList { style: String, body: Vec<AST> },
    OrderedList { head: usize, body: Vec<AST> },
    OrderlessList { body: Vec<AST> },
}

impl Display for ListView {
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
        }
        Ok(())
    }
}
