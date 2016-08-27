use super::*;

pub enum ImageLocal {
    External { source: Url },
    Managed { image: DynamicImage, source: Option<Url> },
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
    #[inline]
    pub fn get_image(&self) -> Option<&DynamicImage> {
        match self {
            Self::External { .. } => None,
            Self::Managed { source: Some(source), .. } => Some(source),
            _ => None,
        }
    }
    #[inline]
    pub fn as_managed(&mut self) -> Result<()> {
        match self {
            Self::External { source } => {
                *self = Self::Managed { image: Reader::open(source.to_file_path()?)?.decode()?, source: Some(*source) };
                Ok(())
            }
            Self::Managed { .. } => Ok(()),
        }
    }
    #[inline]
    pub fn update(&mut self, f: Option<ImageFormat>) -> Result<DynamicImage> {
        let path = self.get_local_path().ok_or(())?.to_file_path()?;
        match f {
            Some(format) => Ok(Reader::open(path)?.decode()?),
            None => Ok(Reader::open(path)?.decode()?),
        }
    }
}
