use dashmap::{mapref::one::Ref, DashMap};
use image::{error::ImageFormatHint, ImageFormat};
use std::collections::hash_map::RandomState;
use url::Url;

pub struct ImageStorage {
    force_backup: bool,
    kv_store: DashMap<String, ImageRecord>,
}

pub struct ImageRecord {
    hash: u64,
    local: ImageLocal,
    www: Option<Url>,
    /// 七牛云
    qi_niu: Option<Url>,
    sm_ms: Option<Url>,
}

pub enum ImageLocal {
    External { source: Url },
    Managed { png: Vec<u8>, source: Option<Url> },
}

impl ImageLocal {
    #[inline]
    pub fn get_path(&self) -> Option<&Url> {
        match self {
            Self::External { source } => Some(source),
            Self::Managed { source: Some(source), .. } => Some(source),
            _ => None,
        }
    }
}

impl ImageRecord {
    #[inline]
    pub fn get_local_path(&self) -> Option<&Url> {
        self.local.get_path()
    }
    #[inline]
    pub fn update(&mut self, f: Option<ImageFormat>) {
        match f {
            Some(format) => ImageReader::open("myimage.png")?.decode()?,
            None => {}
        }
    }
    #[inline]
    pub fn unlink(&mut self) {}
    #[inline]
    pub fn check(&mut self) {}
}

impl ImageStorage {
    pub fn get_image_png(&self, _name: &str) {}
    #[inline]
    pub fn get_image_local_path(&self, name: &str) -> Option<&Url> {
        self.kv_store.get(name).and_then(|f| f.get_local_path())
    }
    pub fn get_image_www() {}
}
