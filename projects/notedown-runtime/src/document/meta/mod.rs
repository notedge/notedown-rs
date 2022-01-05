pub(crate) mod author;
pub(crate) mod datetime;
pub(crate) mod toc;

use super::*;

pub struct DocumentMeta {
    pub title: Option<String>,
    authors: BTreeMap<String, DocumentAuthor>,
    date: Option<DocumentTime>,
    pub toc: TocNode,
}

impl DocumentMeta {
    #[inline]
    pub fn build_toc(&mut self, node: &ASTNode) {
        let cfg = TocConfig::default();
        self.toc = node.toc_configurable(&cfg);
    }
    #[inline]
    pub fn as_lsp_toc(&self, text: &TextIndex) -> DocumentSymbolResponse {
        DocumentSymbolResponse::Nested(vec![self.toc.as_document_symbol(text)])
    }
}
