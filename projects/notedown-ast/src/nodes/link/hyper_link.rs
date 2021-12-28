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
    /// ## Description
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
    pub options: Option<CommandArguments>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HyperLinkTarget {
    Blank,
    Parent,
    This,
    Top,
}

impl HyperLink {
    /// Set description text for the link
    #[inline]
    pub fn set_text(&mut self, msg: impl Into<String>) -> &mut Self {
        self.text = Some(msg.into());
        self
    }
    /// Parse arguments from arguments
    pub fn parse_options(mut self) -> Self {
        let args = match &mut self.options {
            None => return self,
            Some(s) => s,
        };
        args.optional.get_string("text").map(|f| self.set_text(f));
        return self;
    }
}
