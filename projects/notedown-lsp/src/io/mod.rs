use std::fs;
use tower_lsp::lsp_types::Url;

pub fn read_url(url: &Url) -> String {
    match url.to_file_path() {
        Ok(o) => fs::read_to_string(o).unwrap_or_default(),
        Err(_) => String::new(),
    }
}
