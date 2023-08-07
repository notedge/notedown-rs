mod display;
mod paragraph;
pub mod style;
pub mod title;

use crate::hir::ParagraphNode;
use crate::{FontBoldItalicSpan, FontBoldSpan, FontItalicSpan};
use crate::{ast::title::HeadingSpan, CommaNode, NewlineNode,  ParagraphSpaceNode,   PeriodNode, WhitespaceNode};
use deriver::From;
use notedown_error::Url;
use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
};
use crate::hir::{NotedownHIR, NotedownNode};
pub use self::paragraph::{ParagraphTerm, ParagraphSpan};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLiteralSpan {
    pub text: String,
    pub span: Range<u32>,
}

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
    SpaceBreak(Box<ParagraphSpaceNode>),
}

impl NotedownAST {
    pub fn as_hir(&self) -> NotedownHIR {
        let mut terms = Vec::with_capacity(self.terms.len());
        for term in &self.terms {
           match term {
               NotedownTerm::Heading(v) => {
                   terms.push(v.as_hir().into())
               }
               NotedownTerm::Paragraph(v) => {
                   terms.push(v.as_hir().into())
               }
               NotedownTerm::SpaceBreak(_) => {
                  continue
               }
           }
        }
        NotedownHIR { node: terms, url: self.path.clone() }
    }
}



/// `\.`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextEscapeNode {
    pub escape: char,
    pub span: Range<u32>,
}

impl TextLiteralSpan {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { text: body.to_string(), span }
    }
}
