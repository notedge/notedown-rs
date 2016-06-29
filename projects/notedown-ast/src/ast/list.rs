use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ListView {
    QuoteList { style: String, body: Vec<ASTNode> },
    OrderedList { head: usize, body: Vec<ASTNode> },
    OrderlessList { body: Vec<ASTNode> },
    Details { summary: ASTNode, body: Vec<ASTNode> },
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
            Self::Details { .. } => {
                writeln!(f, "Details")?;
            }
        }
        Ok(())
    }
}
