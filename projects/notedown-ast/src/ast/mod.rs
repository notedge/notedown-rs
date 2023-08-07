mod code;
mod display;
mod escaped;
mod paragraph;
mod style;
mod title;
mod whitespace;

pub use self::{
    code::CodeInlineSpan,
    escaped::EscapedCommand,
    paragraph::{ParagraphSpan, ParagraphTerm},
    style::{FontBoldItalicSpan, FontBoldSpan, FontDeleteSpan, FontItalicSpan, FontUnderlineSpan},
    title::HeadingSpan,
    whitespace::{HSpaceNode, IgnoreNode, NewlineSpan, ParagraphBreakSpan, TextSpaceNode, VSpaceNode},
};
use crate::{
    hir::{CodeNode, HeadingLevel, HeadingNode, NotedownHIR, ParagraphNode, TextPlainNode, TextStyleNode},
    CommaNode, PeriodNode,
};
use deriver::From;
use notedown_error::Url;
use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
};

/// The root node of all notedown terms
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotedownAST {
    pub terms: Vec<NotedownTerm>,
    pub path: Option<Url>,
}

#[derive(Clone, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NotedownTerm {
    Heading(Box<HeadingSpan>),
    Paragraph(Box<ParagraphSpan>),
    SpaceBreak(Box<ParagraphBreakSpan>),
}

impl NotedownAST {
    pub fn as_hir(&self) -> NotedownHIR {
        let mut terms = Vec::with_capacity(self.terms.len());
        for term in &self.terms {
            match term {
                NotedownTerm::Heading(v) => terms.push(v.as_hir().into()),
                NotedownTerm::Paragraph(v) => terms.push(v.as_hir().into()),
                NotedownTerm::SpaceBreak(_) => continue,
            }
        }
        NotedownHIR { node: terms, url: self.path.clone() }
    }
}
