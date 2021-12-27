use crate::Url;
use std::fs;

use lspower::lsp::{Diagnostic, TextDocumentContentChangeEvent};
use notedown_rt::NoteVM;
use std::{
    cell::RefCell,
    lazy::{SyncLazy, SyncOnceCell},
    ops::Deref,
};
use tokio::sync::OnceCell;

pub static VM: SyncLazy<NoteVM> = SyncLazy::new(|| NoteVM::default());

pub fn read_url(url: &Url) -> String {
    url.to_file_path().ok().and_then(|e| fs::read_to_string(e).ok()).unwrap_or_default()
}
