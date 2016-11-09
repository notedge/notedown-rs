use std::{collections::VecDeque, fs};
use tower_lsp::lsp_types::{TextDocumentPositionParams, Url};
use unicode_xid::UnicodeXID;
mod global;
pub use global::{initialize_global_storages, FileStateUpdate, FILE_STORAGE};

pub fn read_url(url: &Url) -> String {
    url.to_file_path().ok().and_then(|e| fs::read_to_string(e).ok()).unwrap_or_default()
}

pub fn get_text(tp: TextDocumentPositionParams) -> String {
    let line = tp.position.line as usize;
    let num = tp.position.character as usize;
    read_url(&tp.text_document.uri)
        .lines()
        .nth(line)
        .map(|e| {
            // FIXME: panic when missing sync!!!
            let num = num.min(e.len());
            (&e[..num], &e[num..])
        })
        .map(|(a, b)| get_word(a, b))
        .unwrap_or_default()
}

fn get_word(f: &str, e: &str) -> String {
    let mut v = VecDeque::new();
    for c in f.chars().rev() {
        if c.is_xid_continue() || c == '\\' || c == '<' { v.push_front(c) } else { break }
    }
    for c in e.chars() {
        if c.is_xid_continue() { v.push_back(c) } else { break }
    }
    return v.iter().collect();
}
