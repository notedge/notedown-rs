use super::*;
use notedown_ast::{traits::TocNode, utils::lsp_types::DocumentSymbolParams};

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
}
