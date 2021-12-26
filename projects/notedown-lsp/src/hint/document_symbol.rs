use crate::singleton::read_url;
use lspower::lsp::{DocumentSymbolParams, DocumentSymbolResponse};
use notedown_rt::ParserConfig;

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
