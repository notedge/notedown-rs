pub use self::file_system::VMFileSystem;

use notedown_ast::NoteError;
use std::path::{Path, PathBuf};
use yggdrasil_shared::records::Url;

mod diagnostic;
mod file_system;

pub struct NoteVM {
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
