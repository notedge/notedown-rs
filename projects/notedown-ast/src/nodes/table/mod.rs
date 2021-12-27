use crate::nodes::*;
use std::fmt::Debug;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TableView {
    SimpleTable(Box<SimpleTable>),
}

///
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SimpleTable {
    head: Vec<ASTNode>,
    align: Vec<u8>,
    terms: Vec<Vec<ASTNode>>,
    column: usize,
}
