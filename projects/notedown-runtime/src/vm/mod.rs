use crate::VMFileSystem;
use notedown_ast::{
    traits::{ContextKind, TocNode},
    utils::lsp_types::{Diagnostic, DocumentSymbolResponse, Position, TextDocumentContentChangeEvent, Url},
};
use std::path::Path;

mod diagnostic;

pub struct NoteVM {
    pub fs: VMFileSystem,
}

impl NoteVM {
    pub fn new(root: Url) -> NoteVM {
        Self { fs: VMFileSystem::new(root) }
    }

    pub fn run() {}
    #[inline]
    pub async fn load_cache(&mut self, dump: &Path) {
        let _ = dump;
        todo!()
    }
    #[inline]
    pub async fn dump_cache(&self, dump: &Path) {
        let _ = dump;
        todo!()
    }
}

/// Properties that can be obtained immediately
impl NoteVM {
    #[inline]
    pub fn get_lsp_toc(&self, url: &Url) -> Option<DocumentSymbolResponse> {
        self.fs.get_lsp_toc(url)
    }
}

/// Asynchronous operations that take amount of time
impl NoteVM {
    pub async fn update(&mut self, url: &Url) -> Vec<Diagnostic> {
        self.update_text(url).await;
        self.update_ast(url).await;
        todo!()
    }

    #[inline]
    async fn update_text(&mut self, url: &Url) {
        let _ = url;
        todo!()
    }
    async fn update_ast(&mut self, url: &Url) {
        let _ = url;
        todo!()
    }

    pub fn get_completion_context(&self, url: &Url, p: &Position) -> ContextKind {
        let _ = (url, p);
        todo!()
    }

    #[inline]
    pub async fn update_increment(&mut self, url: &Url, edits: Vec<TextDocumentContentChangeEvent>) {
        let _ = (url, edits);
        todo!()
    }

    pub async fn publish(&self) {}
}
