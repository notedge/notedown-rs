mod image_local;
use crate::Result;
use dashmap::DashMap;
use image::{io::Reader, DynamicImage, ImageFormat};

pub use self::image_local::ImageLocal;
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

impl ImageRecord {
    #[inline]
    pub fn get_local_path(&self) -> Option<&Url> {
        self.local.get_path()
    }
    #[inline]
    pub fn update(&mut self, f: Option<ImageFormat>) -> Result<DynamicImage> {
        let path = self.get_local_path().ok_or(())?.to_file_path()?;
        match f {
            Some(format) => Ok(Reader::open(path)?.decode()?),
            None => Ok(Reader::open(path)?.decode()?),
        }
    }
    #[inline]
    pub fn unlink(&mut self) {}
    #[inline]
    pub fn check(&mut self) {}
}

impl ImageStorage {
    pub fn get_image_png(&self, name: &str) {}
    #[inline]
    pub fn get_image_local_path(&self, name: &str) -> Option<Url> {
        self.kv_store.get(name).and_then(|f| f.get_local_path().cloned())
    }
    pub fn get_image_www() {}
}
