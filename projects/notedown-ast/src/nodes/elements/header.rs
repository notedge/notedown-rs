use crate::nodes::*;

/// ```note
/// \args(id = "custom-id")
/// # head1
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Header {
    ///
    pub level: u8,
    ///
    pub hide_in_toc: bool,
    ///
    pub id: Option<String>,
    ///
    pub children: Vec<ASTNode>,
}

impl Default for Header {
    fn default() -> Self {
        Self { level: 0, hide_in_toc: false, id: None, children: vec![] }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", "#".repeat(self.level as usize))?;
        for term in &self.children {
            write!(f, "{}", term)?
        }
        writeln!(f)?;
        Ok(())
    }
}

impl Header {
    /// Basic constructor
    #[inline]
    pub fn new(children: ASTNodes, level: u8) -> Self {
        let mut new = Self::default();
        let id = children.slugify();
        new.children = children;
        new.id = Some(id);
        new.set_level(level);
        new
    }
    /// Level must between 1 - 6
    #[inline]
    pub fn set_level(&mut self, level: u8) -> &mut Self {
        let level = match level {
            n if n < 1 => 1,
            n if n > 6 => 6,
            n => n,
        };
        self.level = level;
        self
    }
    /// Id must slugify
    #[inline]
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = Some(id.slugify());
        self
    }
}

impl ASTKind {
    /// Construct a header node
    #[inline]
    pub fn header(children: ASTNodes, level: u8, range: MaybeRanged) -> ASTNode {
        Header::new(children, level).into_node(range)
    }
}
