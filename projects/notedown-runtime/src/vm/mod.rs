use crate::vm::file_system::VMFileSystem;
use dashmap::DashSet;
use notedown_ast::{
    utils::{lsp_types::Diagnostic, Url},
    NoteError,
};
use std::path::{Path, PathBuf};

mod file_system;

pub struct VMDiagnostic {
    diag: Vec<NoteError>,
}

pub struct NoteVM {
    diag: VMDiagnostic,
    fs: VMFileSystem,
}

impl NoteVM {
    pub fn run() {}
}
