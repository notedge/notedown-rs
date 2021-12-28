pub mod document;
mod file_system;
mod plugin;
mod plugin_system;
mod vm;

pub use self::{
    file_system::{FileState, VMFileSystem},
    vm::{ContextKind, NoteVM},
};
pub use document::NoteDocument;
pub use plugin_system::{ExtendedPackage, Parser, PluginParser, PluginSystem};
