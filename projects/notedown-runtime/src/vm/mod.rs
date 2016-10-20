use crate::VMFileSystem;
use std::path::Path;
use yggdrasil_shared::records::Url;

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
