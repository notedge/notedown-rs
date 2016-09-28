use tower_lsp::lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};
use crate::io::read_url;
use notedown_parser::parse;
use crate::diagnostic::ToToc;

#[allow(deprecated)]
pub fn document_symbol_provider(args: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
    let nested = match parse(&read_url(&args.text_document.uri)).to_toc().children {
        Some(v) => v,
        None => vec![]
    };
    Some(DocumentSymbolResponse::Nested(nested))
}