use image::ImageError;
use std::io::Error;

pub type Result<T> = std::result::Result<T, NotedownImageError>;

pub enum NotedownImageError {
    IOError(std::io::Error),
    ImageError(ImageError),
    UnknownError,
}

impl From<std::io::Error> for NotedownImageError {
    fn from(e: Error) -> Self {
        Self::IOError(e)
    }
}

impl From<ImageError> for NotedownImageError {
    fn from(e: ImageError) -> Self {
        Self::ImageError(e)
    }
}

impl From<()> for NotedownImageError {
    fn from(_: ()) -> Self {
        Self::UnknownError
    }
}
