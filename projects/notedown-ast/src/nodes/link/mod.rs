mod image_link;
mod hyper_link;

use super::*;
pub use self::image_link::{ImageLink, ImageLayout};

/// 智能链接是指类似 `[ ]` 以及 `[[ ]]` 的结构
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SmartLink {
    /// ## Bare Link
    /// A link without the `[ ]`
    Bare { link: String },
    /// ## EMail
    /// ```note
    /// name@link.net
    /// ```
    EMail { link: String },
    /// ## Path Link
    /// A normal link without any description
    /// ```note
    /// [path-of-link]
    /// ```
    Path { path: String },
    /// ## Normal Link
    /// A normal link without any description
    /// ```note
    /// [alt-text: link]
    /// ```
    PathWithText { text: String, path: String },
    /// ## Two-way link
    /// This means this link will create a reference on the opposite side
    /// ```note
    /// [[link]]
    /// [[link > id]]
    /// ```
    Reciprocal { link: String, id: Option<String> },
    /// ## Tag Block
    /// ```note
    /// [^tag]: text text text
    /// [^tag]:
    ///     text text text
    ///     text text text
    /// ```
    TagBlock { tag: String, text: Vec<String> },
    /// ## Tag Inline
    /// Quote a number while defining
    /// ```note
    /// text [^tag: text text text] text
    /// ```
    TagInline { tag: String, text: Vec<String> },
    /// ## Tag Reference
    /// Quote a number, note that the name is irrelevant, and a number will always be generated in sequence at the end
    /// ```note
    /// text [^tag] text text text
    /// ```
    Reference { tag: String },
    Image(Box<ImageLink>),
}

impl SmartLink {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::LinkNode(box self), range }
    }
}

impl ASTKind {
    #[inline]
    pub fn image_link(self, src: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        ImageLink {
            src: src.into(),
            ..Default::default()
        }.into_node(range)
    }
    #[inline]
    pub fn hyper_link(self, src: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        ImageLink {
            src: src.into(),
            ..Default::default()
        }.into_node(range)
    }



}