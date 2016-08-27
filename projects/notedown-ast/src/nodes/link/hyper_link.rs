use super::*;

#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct HyperLink {
    /// ## Bare Link
    /// A link without the `[ ]`
    pub is_bare: bool,
    /// ## Real Link
    /// A normal link without any description
    /// ```note
    /// [path-of-link]
    /// ```
    pub src: String,
    /// ## Real Link
    /// A normal link without any description
    /// ```note
    /// [alt-text: link]
    /// ```
    pub text: Option<String>,
    ///
    pub download: Option<String>,
    ///
    pub target: Option<HyperLinkTarget>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HyperLinkTarget {
    Blank,
    Parent,
    This,
    Top,
}

impl HyperLink {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        SmartLink::Normal(box self).into_node(range)
    }
}
