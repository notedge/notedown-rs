use crate::{diagnostic::ToToc, io::read_url};
use lspower::lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};
use notedown_parser::ParserConfig;

#[allow(deprecated)]
pub fn document_symbol_provider(args: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
    let cfg = ParserConfig::default();
    let ast = match cfg.parse(&read_url(&args.text_document.uri)) {
        Ok(o) => o,
        Err(_) => return None,
    };

    let nested = match ast.to_toc().children {
        Some(v) => v,
        None => vec![],
    };
    Some(DocumentSymbolResponse::Nested(nested))
}
