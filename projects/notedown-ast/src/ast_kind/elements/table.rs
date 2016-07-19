use crate::ast_kind::*;
use std::fmt::Debug;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TableView {
    SimpleTable { head: Vec<ASTNode>, align: Vec<u8>, terms: Vec<Vec<ASTNode>>, column: usize },
}

impl Display for TableView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SimpleTable { head, align, terms, column } => {
                writeln!(f, "SimpleTable")?;
                writeln!(f, "{:?}", head)?;
                writeln!(f, "{:?}", align)?;
                writeln!(f, "{:?}", terms)?;
                writeln!(f, "{:?}", column)?;
            }
        }
        Ok(())
    }
}
