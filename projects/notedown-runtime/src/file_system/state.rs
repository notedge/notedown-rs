use super::*;
use crate::plugin_system::Parser;
use notedown_ast::{
    traits::TocNode,
    utils::{
        lsp_types::{DocumentSymbolResponse, Url},
        Rope, TextIndex,
    },
};
use std::{fs::read_to_string, path::PathBuf};

pub struct FileState {
    /// used to check weather the file needs re-parse
    fingerprint: u128,
    text: Rope,
    ast: ASTNode,
    meta: FileMeta,
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
        &self.meta.toc
    }
    #[inline]
    pub fn get_lsp_toc(&self) -> DocumentSymbolResponse {
        self.meta.as_lsp_toc(&self.get_text_index())
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
    pub async fn update_ast(&mut self, parse: &Parser) -> Result<()> {
        let text: String = self.text.chars().collect();
        self.meta.clear();
        parse(&text, &mut self.meta).map(|new| self.ast = new)
    }
}
