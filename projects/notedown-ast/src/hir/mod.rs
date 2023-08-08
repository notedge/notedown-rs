mod code;
mod command;
mod heading;
mod link;
mod paragraph;
mod programming;
mod style;

use deriver::From;
use notedown_error::Url;
use std::ops::Range;

pub use self::{
    code::CodeNode,
    command::CommandNode,
    heading::{HeadingLevel, HeadingNode},
    link::UriNode,
    paragraph::{BadNode, ParagraphKind, ParagraphNode},
    programming::IdentifierNode,
    style::{TextEscapeNode, TextPlainNode, TextStyleNode},
};

#[derive(Clone, Debug, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NotedownKind {
    Heading(Box<HeadingNode>),
    Paragraph(Box<ParagraphNode>),
    SyntaxError(Box<BadNode>),
}

///
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotedownHIR {
    pub node: Vec<NotedownKind>,
    pub url: Option<Url>,
}
