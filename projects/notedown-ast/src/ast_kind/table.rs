use super::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TableView<T> {
    SimpleTable { head: Vec<T>, align: Vec<u8>, terms: Vec<Vec<T>>, column: usize },
}

impl<T: Debug> Display for TableView<T> {
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
