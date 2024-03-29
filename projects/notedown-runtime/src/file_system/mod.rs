mod meta;
mod state;

pub use self::{meta::FileMeta, state::FileState};

use async_std::{fs::File, io::ReadExt};
use globset::{Glob, GlobSet, GlobSetBuilder};
use notedown_ast::{
    utils::{lsp_types::Url, DashMap},
    ASTNode, NoteError, Result,
};
use std::path::Path;

pub type Parser = fn(&str, &mut FileMeta) -> Result<ASTNode>;

pub struct VMFileSystem {
    workspace_root: Url,
    file_cache: DashMap<Url, FileState>,
}

impl VMFileSystem {
    #[inline]
    pub fn new(url: Url) -> VMFileSystem {
        Self { workspace_root: url, file_cache: Default::default() }
    }
    #[inline]
    pub fn reset_workspace(&mut self, url: Url) {
        self.workspace_root = url;
    }
    #[inline]
    pub fn clear_cache(&mut self) {
        self.file_cache.clear();
    }
}

impl VMFileSystem {
    #[inline]
    pub async fn update_text(&mut self, url: Url) -> Result<()> {
        let _ = url.to_file_path()?;
        todo!()
    }
    #[inline]
    pub async fn update_ast(&mut self, url: Url, parser: &Parser) -> Result<()> {
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
