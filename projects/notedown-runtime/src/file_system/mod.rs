mod meta;
mod state;

pub use self::{meta::FileMeta, state::FileState};

use async_std::{fs::File, io::ReadExt};
use globset::{Glob, GlobSet, GlobSetBuilder};
use notedown_ast::{
    utils::{
        lsp_types::{DocumentSymbolResponse, Url},
        DashMap,
    },
    ASTNode, NoteError, Result,
};
use std::path::Path;
use crate::plugin_system::Parser;


pub struct VMFileSystem {
    workspace_root: Option<Url>,
    file_cache: DashMap<Url, FileState>,
}

impl Default for VMFileSystem {
    fn default() -> Self {
        Self { workspace_root: None, file_cache: Default::default() }
    }
}

impl VMFileSystem {
    #[inline]
    pub fn new(url: Url) -> VMFileSystem {
        Self { workspace_root: Some(url), file_cache: Default::default() }
    }
    #[inline]
    pub fn set_workspace(&mut self, url: Url) {
        self.workspace_root = Some(url);
    }
    #[inline]
    pub fn clear_cache(&mut self) {
        self.file_cache.clear();
    }
}

impl VMFileSystem {
    pub fn get_lsp_toc(&self, url: &Url) -> Option<DocumentSymbolResponse> {
        match self.file_cache.get(url) {
            None => None,
            Some(s) => Some(s.get_lsp_toc()),
        }
    }
}

impl VMFileSystem {
    #[inline]
    pub async fn update_text(&self, url: &Url) -> Result<()> {
        let _ = url.to_file_path()?;
        // self.file_cache.insert();

        todo!()
    }
    #[inline]
    pub async fn update_ast(&self, url: &Url, parser: &Parser) -> Result<()> {
        match self.file_cache.get_mut(&url) {
            None => Err(NoteError::runtime_error("TODO")),
            Some(mut s) => s.value_mut().update_ast(parser).await,
        }
    }

    /// add a local file path to resolve
    #[inline]
    pub async fn load_path(&mut self, path: &Path) -> Result<()> {
        let mut file = File::open(path).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;
        let _ = Url::from_file_path(path)?;
        todo!()
    }
    #[inline]
    /// add a file url to resolve
    pub async fn load_url(&mut self, url: &Url) -> Result<()> {
        let mut file = File::open(url.to_file_path()?).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;

        todo!()
    }
    #[inline]
    pub async fn load_pattern_text(&mut self, patterns: &str) -> Result<()> {
        let mut builder = GlobSetBuilder::new();
        for row in patterns.lines() {
            builder.add(Glob::new(row)?);
        }
        Ok(self.load_pattern(&builder.build()?).await)
    }
    #[inline]
    pub async fn load_pattern(&mut self, patterns: &GlobSet) {
        let _ = patterns;
        todo!()
    }
}
