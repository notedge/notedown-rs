mod code_block;
mod command;
mod delimiter;
mod header;
mod math;
mod styled;
mod text;

pub use self::{
    code_block::CodeNode,
    command::{Command, CommandKind},
    delimiter::Delimiter,
    header::Header,
    math::{MathKind, MathNode},
    styled::{StyleKind, StyleNode},
    text::TextNode,
};
use super::*;

impl ASTKind {
    #[inline]
    pub fn text(s: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        TextNode::Normal(s.into()).into_node(range)
    }
    #[inline]
    pub fn emoji(s: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        TextNode::emoji(s.into()).into_node(range)
    }
    /// aka `<br>`
    #[inline]
    pub fn hard_break(range: Option<OffsetRange>) -> ASTNode {
        TextNode::HardNewline.into_node(range)
    }
    #[inline]
    pub fn soft_break(range: Option<OffsetRange>) -> ASTNode {
        TextNode::SoftNewline.into_node(range)
    }
    #[inline]
    pub fn strong(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Strong, children }.into_node(range)
    }
    #[inline]
    pub fn italic(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Italic, children }.into_node(range)
    }
    #[inline]
    pub fn emphasis(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Emphasis, children }.into_node(range)
    }
    #[inline]
    pub fn marking(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Highlight, children }.into_node(range)
    }
    #[inline]
    pub fn underline(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Underline, children }.into_node(range)
    }
    #[inline]
    pub fn delete(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Delete, children }.into_node(range)
    }
    #[inline]
    pub fn insert(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Insert, children }.into_node(range)
    }
    #[inline]
    pub fn undercover(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Undercover, children }.into_node(range)
    }
}
