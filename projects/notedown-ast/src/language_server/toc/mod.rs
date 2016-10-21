use super::*;

impl TocNode {
    #[inline]
    pub fn get_lsp_range(&self, text: &TextIndex) -> LSPRange {
        text.get_lsp_range(self.range.start, self.range.end)
    }
    /// Get table of content from element in lsp form
    #[inline]
    #[allow(deprecated)]
    pub fn as_document_symbol(&self, text: &TextIndex) -> DocumentSymbol {
        DocumentSymbol {
            name: "".to_string(),
            detail: Some(self.detail.to_owned()),
            kind: SymbolKind::NAMESPACE,
            tags: None,
            deprecated: None,
            range: self.get_lsp_range(text),
            selection_range: self.get_lsp_range(text),
            children: None,
        }
    }
}
