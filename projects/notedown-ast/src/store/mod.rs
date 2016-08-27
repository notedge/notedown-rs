mod bibtex;

use crate::{store::bibtex::BibliographyStorage, utils::Url};
pub use notedown_image::store::ImageStorage;

pub struct ExternalStorage {
    image: ImageStorage,
    bib: BibliographyStorage,
}

impl ExternalStorage {
    #[inline]
    pub fn get_image_local_path(&self, name: &str) -> Option<Url> {
        self.image.get_image_local_path(name)
    }
}
