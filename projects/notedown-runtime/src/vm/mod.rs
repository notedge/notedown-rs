use crate::VMFileSystem;
use notedown_ast::utils::lsp_types::{Diagnostic, TextDocumentContentChangeEvent, Url};
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

impl NoteVM {
    pub async fn update(&mut self, url: &Url) -> Vec<Diagnostic> {
        self.update_text(url).await;
        self.update_ast(url).await;
        todo!()
    }

    pub async fn update_text(&mut self, url: &Url) {
        // self.fs.update_text();
        todo!()
    }
    pub async fn update_increment(&mut self, url: &Url, edits: Vec<TextDocumentContentChangeEvent>) {
        // self.fs.update_text();
        todo!()
    }

    pub async fn update_ast(&mut self, url: &Url) {}
    pub async fn publish(&self) {}
}
