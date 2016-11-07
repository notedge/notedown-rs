use super::*;
use notedown_ast::{
    traits::TocNode,
    utils::{lsp_types::Diagnostic, TextIndex},
};

pub struct FileMeta {
    pub error: Vec<NoteError>,
    pub warn: Vec<NoteError>,
    pub info: Vec<NoteError>,
    pub hint: Vec<NoteError>,
    pub toc: TocNode,
}

impl FileMeta {
    #[inline]
    pub fn clear(&mut self) {
        self.error.clear();
        self.toc = TocNode::default()
    }
}

pub enum DiagnosticSeverity2 {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

pub enum DiagnosticTag2 {
    /// Unused or unnecessary code.
    /// Clients are allowed to render diagnostics with this tag faded out instead of having an error squiggle.
    Unnecessary,
    /// Deprecated or obsolete code.
    /// Clients are allowed to rendered diagnostics with this tag strike through.
    Deprecated,
}

impl FileMeta {
    pub fn as_lsp_diagnostic(&self, index: &TextIndex) -> Diagnostic {
        let out = Vec::with_capacity(self.error.len() + self.hint.len() + self.warn.len() + self.info.len());
        for i in self.error {
            Diagnostic { range: Default::default(), severity: None, code: None, code_description: None, source: index.get_lsp_range(i.range), message: "".to_string(), related_information: None, tags: None, data: None }
        }
        unimplemented!()
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
    fn toc_lsp_configurable(&self, config: &TocConfig, index: &TextIndex) -> DocumentSymbol {
        let nodes = self.toc_configurable(config);
        DocumentSymbol {
            //
            name: "".to_string(),
            detail: Some(nodes.detail),
            kind: SymbolKind::NAMESPACE,
            tags: None,
            deprecated: None,
            range: Default::default(),
            selection_range: Default::default(),
            children: None,
        }
    }
}
