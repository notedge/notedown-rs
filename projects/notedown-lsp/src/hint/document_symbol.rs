use crate::VM;
use lspower::lsp::{DocumentSymbolParams, DocumentSymbolResponse};

#[inline]
pub fn document_symbol_provider(args: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
    VM.get_lsp_toc(&args.text_document.uri)
}
