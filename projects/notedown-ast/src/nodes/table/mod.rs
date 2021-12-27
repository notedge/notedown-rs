use crate::nodes::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum TableView {
    SimpleTable(Box<SimpleTable>),
}

///
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SimpleTable {
    pub head: Vec<ASTNode>,
    pub align: Vec<u8>,
    pub terms: Vec<Vec<ASTNode>>,
    pub column: usize,
}
