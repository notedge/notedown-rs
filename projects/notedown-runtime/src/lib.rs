pub mod document;
mod file_system;
mod plugin;
mod plugin_system;
mod vm;

pub use self::{
    file_system::{FileMeta, FileState, VMFileSystem},
    vm::{ContextKind, NoteVM},
};
mod errors;
pub use errors::{DiagnosticLevel, NoteError, NoteErrorKind, Result};
pub use plugin_system::{ExtendedPackage, Parser, PluginParser, PluginSystem};
