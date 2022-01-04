use super::*;

pub struct DocumentMeta {
    pub title: Option<String>,
    pub author: Vec<DocumentAuthor>,
    pub date: Option<DocumentDate>,
    pub toc: TocNode,
}

pub struct DocumentAuthor {
    name: String,
    email: Option<String>,
}

pub enum DocumentDate {
    /// `\date: runtime-today`
    RuntimeToday,
    /// `\date: file-changed`
    FileChanged,
    /// `\date: file-created`
    FileCreated,
    /// `\date: git-changed`
    GitChanged,
    /// `\date: git-created`
    GitCreated,
    DateTime(NaiveDateTime),
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
