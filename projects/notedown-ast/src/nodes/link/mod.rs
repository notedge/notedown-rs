mod hyper_link;
mod image_link;
mod other;
mod reference;
mod two_way;

pub use self::{
    hyper_link::{HyperLink, HyperLinkTarget},
    image_link::{ImageLayout, ImageLink},
    other::EmailLink,
    reference::TagReference,
    two_way::TwoWayLink,
};
use super::*;

/// 智能链接是指类似 `[ ]` 以及 `[[ ]]` 的结构
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SmartLink {
    EMail(Box<EmailLink>),
    Normal(Box<HyperLink>),
    Image(Box<ImageLink>),
    TwoWay(Box<TwoWayLink>),
    Reference(Box<TagReference>),
}

impl SmartLink {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::LinkNode(self), range }
    }
}

impl ASTKind {
    #[inline]
    pub fn image_link(src: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        ImageLink { src: src.into(), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn image_link_alt(src: impl Into<String>, alt: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        ImageLink { src: src.into(), alt: Some(alt.into()), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn hyper_link(src: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        HyperLink { src: src.into(), is_bare: false, ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn hyper_link_text(src: impl Into<String>, text: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        HyperLink { src: src.into(), is_bare: false, text: Some(text.into()), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn bare_link(src: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        HyperLink { src: src.into(), is_bare: true, ..Default::default() }.into_node(range)
    }
}
