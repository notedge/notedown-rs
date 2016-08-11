mod hyper_link;
mod image_link;

pub use self::image_link::{ImageLayout, ImageLink};
use super::*;
use crate::nodes::link::hyper_link::HyperLink;

/// 智能链接是指类似 `[ ]` 以及 `[[ ]]` 的结构
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SmartLink {
    EMail(Box<EmailLink>),
    Normal(Box<HyperLink>),
    Image(Box<ImageLink>),
    TwoWay(Box<TwoWayLink>),
    Reference(Box<TagReference>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EmailLink {
    name: String
}

/// ## Two-way link
/// This means this link will create a reference on the opposite side
/// ```note
/// [[link]]
/// [[link > id]]
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TwoWayLink {
    link: String,
    id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TagReference {
    /// ## Tag Block
/// ```note
/// [^tag]: text text text
/// [^tag]:
///     text text text
///     text text text
/// ```
    pub inline: bool,
    /// ## Tag Reference
/// Quote a number, note that the name is irrelevant, and a number will always be generated in sequence at the end
/// ```note
/// text [^tag] text text text
/// ```
    pub tag: String,
    /// ## Tag Inline
/// Quote a number while defining
/// ```note
/// text [^tag: text text text] text
/// ```
    pub text: Vec<String>,
}

impl SmartLink {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::LinkNode(self), range }
    }
}

impl ASTKind {
    #[inline]
    pub fn image_link(self, src: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        ImageLink { src: src.into(), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn image_link_alt(self, src: impl Into<String>, alt: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        ImageLink { src: src.into(), alt: Some(alt.into()), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn hyper_link(self, src: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        HyperLink { src: src.into(), is_bare: false, ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn hyper_link_text(self, src: impl Into<String>, text: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        HyperLink { src: src.into(), is_bare: false, text: Some(text.into()), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn bare_link(self, src: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        HyperLink { src: src.into(), is_bare: true, ..Default::default() }.into_node(range)
    }
}
