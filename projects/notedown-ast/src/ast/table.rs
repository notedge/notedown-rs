use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TableView {
    SimpleTable { head: Vec<AST>, align: Vec<u8>, terms: Vec<Vec<AST>>, column: usize },
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
