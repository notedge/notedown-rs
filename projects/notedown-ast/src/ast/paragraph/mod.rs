use crate::hir::ParagraphKind;
use super::*;
mod display;


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphSpan {
    pub terms: Vec<ParagraphTerm>,
    pub span: Range<u32>,
}

#[derive(Clone, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParagraphTerm {
    /// Normal ast with white space
    Text(Box<TextPlainNode>),
    WhiteSpace(Box<WhitespaceNode>),
    /// `*italic*`
    Italic(Box<FontItalicSpan>),
    /// `**bold**`
    Bold(Box<FontBoldSpan>),
    /// `**bold italic**`
    BoldItalic(Box<FontBoldItalicSpan>),
    NewLine(Box<NewlineNode>),
    Comma(Box<CommaNode>),
    Period(Box<PeriodNode>),
    Escape(Box<TextEscapeNode>),
}


impl ParagraphSpan {
    pub fn as_hir(&self) -> ParagraphNode {
        let mut terms: Vec<ParagraphKind> = Vec::with_capacity(self.terms.len());
        for term in &self.terms {
            match term {
                ParagraphTerm::Text(v) => {
                    terms.push(v.as_ref().clone().into())
                }
                ParagraphTerm::WhiteSpace(v) => {
                    // terms.push(v.as_hir().into());
                }
                ParagraphTerm::Italic(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::Bold(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::BoldItalic(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::NewLine(v) => {
                    // terms.push(v.as_hir().into());
                }
                ParagraphTerm::Comma(v) => {
                    // terms.push(v.as_hir().into());
                }
                ParagraphTerm::Period(v) => {
                    // terms.push(v.as_hir().into());
                }
                ParagraphTerm::Escape(v) => {
                    // terms.push(v.as_hir().into());
                }
            }
        }
        ParagraphNode { terms, span: self.span.clone() }
    }
}