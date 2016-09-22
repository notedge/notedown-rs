use dashmap::DashSet;
use notedown_ast::utils::Url;
use std::path::{Path, PathBuf};

pub struct VM {}

pub struct VMFileSystem {
    root: Url,
    files: DashSet<Url>,
}

impl VMFileSystem {
    pub fn new() {}
    pub fn load(&self) {}
}

impl VMFileSystem {
    pub fn insert_path(&mut self, _: PathBuf) {}
    pub fn insert_url(&mut self, _: PathBuf) {}
    pub fn insert_pattern() {}

    pub fn remove() {}
    pub fn remove_files() {}
}
