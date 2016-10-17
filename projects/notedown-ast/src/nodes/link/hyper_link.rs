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
    ///
    pub options: Option<CommandOptions>,
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
    pub fn into_node(self, range: MaybeRanged) -> ASTNode {
        SmartLink::Normal(box self).into_node(range)
    }
    #[inline]
    pub fn set_text(&mut self, msg: impl Into<String>) {
        self.text = Some(msg.into());
    }

    pub fn parse_options(mut self) -> Self {
        let options = match &mut self.options {
            None => return self,
            Some(s) => s,
        };
        options.kvs.get_string("text").map(|f| self.set_text(f));
        return self;
    }
}
