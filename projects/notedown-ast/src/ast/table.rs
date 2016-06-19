use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TableView {
    SimpleTable {
        head: Vec<AST>,
        align: Vec<u8>,
        terms: Vec<Vec<AST>>,
        column: usize,
    },
}

impl Display for TableView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}