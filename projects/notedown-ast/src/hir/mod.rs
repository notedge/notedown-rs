mod heading;
mod paragraph;

use std::ops::Range;
use deriver::From;
use notedown_error::Url;
use crate::ast::title::{HeadingLevel, HeadingSpan};

pub use self::paragraph::{ParagraphNode, TextStyleNode, ParagraphKind};
pub use self::heading::HeadingNode;
pub use self::paragraph::TextPlainNode;

#[derive(Clone, Debug, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NotedownNode {
    Heading(Box<HeadingNode>),
    Paragraph(Box<ParagraphNode>),
}

///
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotedownRoot {
    pub node: NotedownNode,
    pub range: Range<u32>,
}

///
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotedownHIR {
    pub node: Vec<NotedownNode>,
    pub url: Option<Url>,
}