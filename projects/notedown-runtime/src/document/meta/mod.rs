use super::*;

pub struct DocumentMeta {
    pub title: Option<String>,
    pub author: Vec<String>,
    pub date: Option<NaiveDateTime>,
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
