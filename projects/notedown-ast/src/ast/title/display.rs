use super::*;

impl From<usize> for HeadingLevel {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Article,
            1 => Self::Header1,
            2 => Self::Header2,
            3 => Self::Header3,
            4 => Self::Header4,
            5 => Self::Header5,
            _ => Self::Header6,
        }
    }
}


impl Display for HeadingSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
