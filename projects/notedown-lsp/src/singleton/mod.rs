use crate::Url;
use std::fs;

use lspower::lsp::{Diagnostic, TextDocumentContentChangeEvent};
use notedown_rt::NoteVM;
use std::{
    lazy::{SyncLazy, SyncOnceCell},
    ops::Deref,
};
use std::cell::RefCell;
use tokio::sync::OnceCell;

pub static VM: SyncLazy<SingletonVM> = SyncLazy::new(|| SingletonVM::default());

#[derive(Default)]
pub struct SingletonVM {
    inner: RefCell<NoteVM>,
}

impl Deref for SingletonVM {
    type Target = NoteVM;

    fn deref(&self) -> &Self::Target {
        self.vm()
    }
}

impl SingletonVM {
    fn vm(&self) -> &mut NoteVM {
        match self.inner.get_mut() {
            None => self.inner.set(),
            Some(s) => return s,
        }
       self.inner.get_mut()

        self.inner.get_or_init(|| NoteVM::default());
    }

    pub async fn update(&self, url: &Url) -> Vec<Diagnostic> {
        self.vm().update(url).await
    }
    pub async fn update_increment(&self, url: &Url, edits: Vec<TextDocumentContentChangeEvent>) -> Vec<Diagnostic> {
        self.vm().update_increment(url, edits).await
    }
    pub fn gc_mark(&self, url: &Url) {
        todo!()
    }
}

pub fn read_url(url: &Url) -> String {
    url.to_file_path().ok().and_then(|e| fs::read_to_string(e).ok()).unwrap_or_default()
}
