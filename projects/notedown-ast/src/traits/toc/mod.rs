use crate::{nodes::ASTKind, traits::Slugify, ASTNode};
use std::ops::Range;
use yggdrasil_shared::records::{
    lsp_types::{DocumentSymbol, SymbolKind},
    TextIndex,
};

mod visit_ast;

pub trait TableOfContent {
    fn table_of_content(&self, config: &TableConfig) -> TableNode;
    fn table_of_content_lsp(&self, config: &TableConfig, _text: &TextIndex) -> DocumentSymbol {
        let nodes = self.table_of_content(config);
        return DocumentSymbol::from(nodes);
    }
}

#[derive(Debug)]
pub struct TableNode {
    pub level: u8,
    pub detail: String,
    pub range: Range<usize>,
    pub children: Vec<TableNode>,
}

pub struct TableConfig {
    pub max_depth: u8,
}

impl Default for TableNode {
    fn default() -> Self {
        Self { level: 0, detail: String::from("ROOT"), range: Default::default(), children: vec![] }
    }
}
