use crate::{utils::Url, ExternalStorage};
use dashmap::DashMap;

pub struct ImageStorage {
    force_backup: bool,
    kv_store: DashMap<String, String>,
}

pub struct ImageRecord {
    hash: u64,
    png: ImageLocal,
    local: Option<Url>,
    www: Option<Url>,
    /// 七牛云
    qi_niu: Option<Url>,
    sm_ms: Option<Url>,
}

pub enum ImageLocal {
    Path(Url),
    Binary(Vec<u8>),
}

impl ImageStorage {
    pub fn get_image_png(&self, _name: &str) {}

    pub fn get_image_local_path(&self, name: &str) {}
    pub fn get_image_www() {}
}

impl ExternalStorage {
    pub fn get_image_local_path() {}
    pub fn get_image_custom_path() {}
}
