mod heading;


use std::ops::Range;
use crate::ParagraphNode;
use crate::ast::title::{HeadingLevel, HeadingSpan};
use crate::hir::heading::HeadingNode;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NotedownNode {
    Heading(Box<HeadingNode>)
}

///
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotedownHIR {
    pub node: NotedownNode,
    pub range: Range<u32>,
}