mod diagnostic;

pub use notedown_ast::traits::ContextKind;

use crate::{
    plugin_system::{Parser, PluginSystem},
    VMFileSystem,
};
use std::path::Path;
use yggdrasil_shared::records::{
    lsp_types::{Diagnostic, DocumentSymbolResponse, Position, TextDocumentContentChangeEvent},
    Url,
};

pub struct NoteVM {
    fs: VMFileSystem,
    ps: PluginSystem,
}

impl Default for NoteVM {
    fn default() -> Self {
        Self { fs: Default::default(), ps: Default::default() }
    }
}

impl NoteVM {
    pub fn new(root: Url) -> NoteVM {
        Self { fs: VMFileSystem::new(root), ps: Default::default() }
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
    /// Remove cache that no longer using
    #[inline]
    pub fn gc(&self) {
        self.fs.cache.retain(|_, v| v.can_gc())
    }
    /// Mark some file is useless
    #[inline]
    pub fn gc_mark(&self, _: &Url) {
        // TODO: mark
    }

    #[inline]
    pub fn get_lsp_toc(&self, url: &Url) -> Option<DocumentSymbolResponse> {
        let toc = match self.fs.cache.get(url) {
            None => return None,
            Some(s) => s.get_lsp_toc(),
        };
        Some(DocumentSymbolResponse::Nested(vec![toc]))
    }
}

/// Asynchronous operations that take amount of time
impl NoteVM {
    #[inline]
    pub async fn update(&self, url: &Url) -> Vec<Diagnostic> {
        self.update_text(url).await;
        match self.ps.get_parser("note") {
            None => {}
            Some(parser) => {
                self.update_ast(url, &parser).await;
            }
        }
        todo!()
    }

    #[inline]
    async fn update_text(&self, url: &Url) -> bool {
        match self.fs.update_text(url).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    #[inline]
    async fn update_ast(&self, url: &Url, parser: &Parser) -> bool {
        match self.fs.update_ast(url, parser).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_completion_context(&self, url: &Url, p: &Position) -> ContextKind {
        let _ = (url, p);
        todo!()
    }

    #[inline]
    pub async fn update_increment(&self, url: &Url, edits: Vec<TextDocumentContentChangeEvent>) -> Vec<Diagnostic> {
        let _ = (url, edits);
        todo!()
    }

    pub async fn publish(&self) {}
}
