use dashmap::{mapref::one::Ref, DashMap, DashSet};
use notedown_ast::{
    utils::{lsp_types::TextEdit, Url},
    ASTNode, NoteError, Result,
};
use std::path::PathBuf;

pub struct VMFileSystem {
    workspace_root: Url,
    cache_raw: DashMap<Url, String>,
    cache_ast: DashMap<Url, ASTNode>,
}

impl VMFileSystem {
    pub fn new(url: Url) -> VMFileSystem {
        Self { workspace_root: url, cache_raw: Default::default(), cache_ast: Default::default() }
    }
    pub fn reset_workspace(&mut self, url: Url) {
        self.workspace_root = url;
    }
    pub fn clear_cache(&mut self) {
        self.cache_ast.clear();
        self.cache_raw.clear();
    }
}

impl VMFileSystem {
    pub async fn update_text(&mut self, url: Url) -> Result<()> {
        let _ = url.to_file_path()?;
        todo!()
    }
    pub async fn update_ast(&mut self, url: Url, parser: fn(&str) -> Result<ASTNode>) -> Result<()> {
        match self.cache_raw.get(&url) {
            None => Err(NoteError::runtime_error("TODO")),
            Some(s) => match parser(s.value()) {
                Ok(ast) => {
                    self.cache_ast.insert(url, ast);
                    Ok(())
                }
                Err(e) => Err(e),
            },
        }
    }

    /// add a local file path to resolve
    pub fn insert_path(&mut self, path: PathBuf) {}
    pub fn insert_url(&mut self, _: PathBuf) {}
    pub fn insert_pattern() {}

    pub fn remove() {}
    pub fn remove_files() {}
}
