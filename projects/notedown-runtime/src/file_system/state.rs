use crate::{
    document::{NoteDocument, TocNode},
    plugin_system::Parser,
};
use notedown_error::Result;
use std::fs::read_to_string;
#[cfg(feature = "lsp")]
use yggdrasil_shared::records::lsp_types::DocumentSymbol;
use yggdrasil_shared::records::{Rope, TextIndex, Url};

#[derive(Debug, Clone)]
pub struct FileState {
    /// used to check weather the file needs re-parse
    fingerprint: u128,
    text: Rope,
    document: NoteDocument,
}

impl PartialEq for FileState {
    fn eq(&self, other: &Self) -> bool {
        self.fingerprint.eq(&other.fingerprint)
    }
}

impl FileState {
    #[inline]
    pub fn get_text(&self) -> String {
        self.text.chars().collect()
    }

    #[inline]
    pub fn get_text_index(&self) -> TextIndex {
        TextIndex::new(self.get_text())
    }
    #[inline]
    pub fn get_toc(&self) -> &TocNode {
        &self.document.get_toc()
    }
    #[cfg(feature = "lsp")]
    #[inline]
    pub fn get_lsp_toc(&self) -> DocumentSymbol {
        self.document.as_lsp_toc(&self.get_text_index())
    }
    #[inline]
    pub fn can_gc(&self) -> bool {
        false
    }
}

impl FileState {
    pub async fn load_local(&mut self, url: &Url) -> Result<()> {
        let path = url.to_file_path()?;
        self.text = Rope::from_str(&read_to_string(path)?);
        Ok(())
    }
    pub async fn load_remote(&mut self, url: &Url) {
        todo!("Remote: {}", url)
    }
    #[inline]
    pub async fn update_document(&mut self, parse: &Parser) -> Result<()> {
        let text: String = self.text.chars().collect();
        let mut errors = vec![];
        let parsed = parse(&text, &mut errors)?;
        todo!()
    }
}
