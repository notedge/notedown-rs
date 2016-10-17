pub use self::file_system::VMFileSystem;

use notedown_ast::{
    utils::{lsp_types::Diagnostic, Url},
    NoteError,
};
use std::path::{Path, PathBuf};

mod diagnostic;
mod file_system;

pub struct VMDiagnostic {
    diag: Vec<NoteError>,
}

impl VMDiagnostic {
    pub async fn publish_diagnostic_lsp() -> Diagnostic {}
    pub async fn publish_hint_lsp() {}
}

pub struct NoteVM {
    pub hints: VMDiagnostic,
    pub fs: VMFileSystem,
}

impl NoteVM {
    pub fn new(root: Url) -> NoteVM {
        Self { hints: VMDiagnostic { diag: vec![] }, fs: VMFileSystem::new(root) }
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
