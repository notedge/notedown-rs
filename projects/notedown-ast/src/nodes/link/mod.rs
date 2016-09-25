mod hyper_link;
mod image_link;
mod other;
mod rd;
mod reference;
mod two_way;

pub use self::{
    hyper_link::{HyperLink, HyperLinkTarget},
    image_link::{ImageLayout, ImageLink},
    other::EmailLink,
    rd::ResourceDescriptor,
    reference::TagReference,
    two_way::TwoWayLink,
};
use super::*;
use crate::command::CommandOptions;

/// 智能链接是指类似 `[ ]` 以及 `[[ ]]` 的结构
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum SmartLink {
    /// - `[<RD>]`: Resource Descriptor
    ///
    /// ```note
    /// [./relative-path]: 使用相对路径
    /// [file://]: 使用绝对路径
    /// [https://]: 使用远程 url 路径
    /// [id/path]: 使用默认储存库
    /// [@storage/id/path]: 使用具体的某个储存库
    /// ```
    ExternalResource(Box<ResourceDescriptor>),
    /// ```note
    /// [name@example.com](options)
    /// ```
    EMail(Box<EmailLink>),
    /// ```note
    /// [<RD>](options)
    /// [describe][<RD>](options)
    /// ```
    Normal(Box<HyperLink>),
    /// ```note
    /// [<RD>](options)
    /// [!image-alt][<RD>](opts)
    /// ```
    Image(Box<ImageLink>),
    /// ## Tag Definition Block
    /// ```note
    /// [^tag-define]:
    ///     text text text
    ///     text text text
    /// ```
    /// ## Tag Reference Inline
    /// ```note
    /// [<RD>](opts)
    /// [^tag-name](options)
    /// [^tag-name][<RD>](opts)
    /// ```
    Reference(Box<TagReference>),
    /// ```note
    /// [[<RD>](options)]
    /// [[text][<RD>](opts)]
    /// ```
    TwoWay(Box<TwoWayLink>),
}

impl SmartLink {
    #[inline]
    pub fn into_node(self, range: MaybeRanged) -> ASTNode {
        ASTNode { value: ASTKind::LinkNode(self), range }
    }
}

impl ASTKind {
    #[inline]
    pub fn image_link(src: impl Into<String>, range: MaybeRanged) -> ASTNode {
        ImageLink { source: src.into(), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn image_link_alt(src: impl Into<String>, alt: impl Into<String>, range: MaybeRanged) -> ASTNode {
        ImageLink { source: src.into(), description: Some(alt.into()), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn hyper_link(src: impl Into<String>, range: MaybeRanged) -> ASTNode {
        HyperLink { src: src.into(), is_bare: false, ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn hyper_link_text(src: impl Into<String>, text: impl Into<String>, range: MaybeRanged) -> ASTNode {
        HyperLink { src: src.into(), is_bare: false, text: Some(text.into()), ..Default::default() }.into_node(range)
    }
    #[inline]
    pub fn bare_link(src: impl Into<String>, range: MaybeRanged) -> ASTNode {
        HyperLink { src: src.into(), is_bare: true, ..Default::default() }.into_node(range)
    }
}
