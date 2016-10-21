mod diagnostic;
mod toc;

use crate::{errors::DiagnosticLevel, traits::TocNode, NoteError};
use yggdrasil_shared::records::{
    lsp_types::{Diagnostic, DiagnosticSeverity, DiagnosticTag, DocumentSymbol, SymbolKind},
    LSPRange, TextIndex,
};
