use super::*;


#[derive(Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct HyperLink {
    /// ## Bare Link
/// A link without the `[ ]`
    pub bare: bool,
    /// A normal link without any description
    /// ```note
    /// [path-of-link]
    /// ```
    pub src: String,
    /// A normal link without any description
/// ```note
/// [alt-text: link]
/// ```
    pub text: Option<String>,
}


impl HyperLink {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        SmartLink::Image(box self).into_node(range)
    }
    #[inline]
    pub fn set_alt(mut self, msg: impl Into<String>) -> Self {
        self.alt = Some(msg.into());
        return self;
    }
    #[inline]
    pub fn set_size(mut self, width: usize, height: usize) -> Self {
        self.size = Some((width, height));
        return self;
    }
    #[inline]
    pub fn set_layout(mut self, layout: ImageLayout) -> Self {
        self.layout = Some(layout);
        return self;
    }
}
