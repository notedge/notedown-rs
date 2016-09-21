use tower_lsp::lsp_types::{DocumentSymbolParams, DocumentSymbol, SymbolKind, DocumentSymbolResponse};
use crate::io::read_url;
use notedown_parser::parse;
use crate::diagnostic::ToToc;

#[allow(deprecated)]
pub fn document_symbol_provider(args: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
    let toc = parse(&read_url(&args.text_document.uri)).to_toc();

    let tree: Vec<DocumentSymbol> = vec![DocumentSymbol {
        name: String::from("TOC"),
        detail: Some(String::from("Table of Contents")),
        kind: SymbolKind::Package,
        deprecated: None,
        range: Default::default(),
        selection_range: Default::default(),
        children: Some(vec![toc]),
    }];
    Some(DocumentSymbolResponse::Nested(tree))
}