use super::*;
use crate::hir::ParagraphKind;
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
    WhiteSpace(Box<TextSpaceNode>),
    /// `*italic*`
    Italic(Box<FontItalicSpan>),
    /// `**bold**`
    Bold(Box<FontBoldSpan>),
    /// `**bold italic**`
    BoldItalic(Box<FontBoldItalicSpan>),
    /// `_underline_`
    Underline(Box<FontUnderlineSpan>),
    /// `__delete__`
    Delete(Box<FontDeleteSpan>),
    /// `` `code` ``
    Code(Box<CodeInlineSpan>),
    NewLine(Box<NewlineSpan>),
    Comma(Box<CommaNode>),
    Period(Box<PeriodNode>),
    Escape(Box<TextEscapeSpan>),
}

impl ParagraphSpan {
    pub fn as_hir(&self) -> ParagraphNode {
        let mut terms: Vec<ParagraphKind> = Vec::with_capacity(self.terms.len());
        for term in &self.terms {
            match term {
                ParagraphTerm::Text(v) => terms.push(v.as_ref().clone().into()),
                ParagraphTerm::WhiteSpace(v) => {
                    terms.push(ParagraphKind::Space(v.clone()));
                }
                ParagraphTerm::Italic(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::Bold(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::BoldItalic(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::NewLine(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::Comma(v) => {
                    todo!()
                    // terms.push(v.as_hir().into());
                }
                ParagraphTerm::Period(v) => {
                    todo!()
                    // terms.push(v.as_hir().into());
                }
                ParagraphTerm::Escape(v) => {
                    todo!()
                    // terms.push(v.as_hir().into());
                }
                ParagraphTerm::Underline(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::Delete(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::Code(v) => terms.push(v.as_hir().into()),
            }
        }
        ParagraphNode { terms, span: self.span.clone() }
    }
}
