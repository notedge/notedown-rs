mod bibtex;
mod image;

use self::image::ImageStorage;
use crate::store::bibtex::BibliographyStorage;

pub struct ExternalStorage {
    image: ImageStorage,
    bib: BibliographyStorage,
}
