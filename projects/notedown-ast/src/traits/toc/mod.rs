use crate::{nodes::ASTKind, traits::Slugify, ASTNode};
use std::ops::Range;
use yggdrasil_shared::records::{
    lsp_types::{DocumentSymbol, SymbolKind},
    TextIndex,
};

mod visit_ast;

pub trait TableOfContent {
    fn toc_configurable(&self, config: &TableConfig) -> TableNode;
    #[inline]
    fn toc(&self) -> TableNode {
        let cfg = TableConfig::default();
        self.toc_configurable(&cfg)
    }
    #[inline]
    fn toc_lsp(&self, text: &TextIndex) -> DocumentSymbol {
        let cfg = TableConfig::default();
        self.toc_lsp_configurable(&cfg, text)
    }
    #[inline]
    fn toc_lsp_configurable(&self, config: &TableConfig, _text: &TextIndex) -> DocumentSymbol {
        let nodes = self.toc_configurable(config);
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

impl Default for TableConfig {
    fn default() -> Self {
        Self { max_depth: u8::MAX }
    }
}

impl Default for TableNode {
    fn default() -> Self {
        Self { level: 0, detail: String::from("ROOT"), range: Default::default(), children: vec![] }
    }
}
