use std::fs;
use tower_lsp::lsp_types::{TextDocumentPositionParams, Url};

pub fn read_url(url: &Url) -> String {
    url.to_file_path().ok().and_then(|e| fs::read_to_string(e).ok()).unwrap_or_default()
}

pub fn get_text(tp: TextDocumentPositionParams) -> Option<String> {
    tp.text_document.uri.to_file_path().ok().and_then(|e| fs::read_to_string(e).ok())
}
