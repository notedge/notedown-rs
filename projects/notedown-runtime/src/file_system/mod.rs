mod get_env;
mod state;

pub use self::state::FileState;

use crate::plugin_system::Parser;
use async_std::{fs::File, io::ReadExt};
use globset::{Glob, GlobSet, GlobSetBuilder};
use notedown_error::{NoteError, Result};
use std::path::Path;
use yggdrasil_shared::records::{DashMap, Url};

pub struct VMFileSystem {
    pub workspace_root: Option<Url>,
    pub cache: DashMap<Url, FileState>,
}

impl Default for VMFileSystem {
    fn default() -> Self {
        Self { workspace_root: None, cache: Default::default() }
    }
}

impl VMFileSystem {
    #[inline]
    pub fn new(url: Url) -> VMFileSystem {
        Self { workspace_root: Some(url), cache: Default::default() }
    }
    #[inline]
    pub fn set_workspace(&mut self, url: Url) {
        self.workspace_root = Some(url);
    }
    #[inline]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl VMFileSystem {}

impl VMFileSystem {
    #[inline]
    pub async fn update_text(&self, url: &Url) -> Result<()> {
        let _ = url.to_file_path()?;
        // self.file_cache.insert();

        todo!()
    }
    #[inline]
    pub async fn update_ast(&self, url: &Url, parser: &Parser) -> Result<()> {
        match self.cache.get_mut(&url) {
            None => Err(NoteError::runtime_error("TODO")),
            Some(mut s) => s.value_mut().update_document(parser).await,
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
