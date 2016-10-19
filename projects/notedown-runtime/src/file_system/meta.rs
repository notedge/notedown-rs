use super::*;
use notedown_ast::traits::TocNode;
use yggdrasil_shared::records::{
    lsp_types::{Diagnostic, DocumentSymbol, DocumentSymbolParams, SymbolKind},
    TextIndex,
};

pub struct FileMeta {
    diag: Vec<NoteError>,
    toc: TocNode,
}

impl FileMeta {
    #[inline]
    pub fn clear(&mut self) {
        self.diag.clear();
        self.toc = TocNode::default()
    }
}

impl FileMeta {
    pub fn as_lsp_diagnostic(&self) -> Diagnostic {
        todo!()
    }
    pub fn as_lsp_toc(&self) -> DocumentSymbolParams {
        todo!()
    }
    /// Get table of content from element in lsp form
    #[inline]
    fn toc_lsp(&self, text: &TextIndex) -> DocumentSymbol {
        let cfg = TocConfig::default();
        self.toc_lsp_configurable(&cfg, text)
    }
    /// Get table of content from element in lsp form with config
    #[inline]
    fn toc_lsp_configurable(&self, config: &TocConfig, _text: &TextIndex) -> DocumentSymbol {
        let nodes = self.toc_configurable(config);
        return DocumentSymbol::from(nodes);
    }
}

impl From<TocNode> for DocumentSymbol {
    #[allow(deprecated)]
    fn from(node: TocNode) -> Self {
        DocumentSymbol { name: "".to_string(), detail: Some(node.detail), kind: SymbolKind::NAMESPACE, tags: None, deprecated: None, range: Default::default(), selection_range: Default::default(), children: None }
    }
}
