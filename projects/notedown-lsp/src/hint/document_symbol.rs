use crate::singleton::read_url;
use lspower::lsp::{DocumentSymbolParams, DocumentSymbolResponse};
use crate::VM;

#[allow(deprecated)]
pub fn document_symbol_provider(args: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
    match VM.get_lsp_toc(&args.text_document.uri) {
        None => {}
        Some(s) => {}
    }

    VM.get_lsp_toc(&args.text_document.uri).map(|f| f.as_document_symbol())

    Some(DocumentSymbolResponse::Nested(nested))
}
