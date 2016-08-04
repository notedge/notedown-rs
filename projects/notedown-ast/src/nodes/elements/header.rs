use crate::nodes::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Header {
    pub level: u8,
    pub children: Vec<ASTNode>,
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", "#".repeat(self.level as usize),)?;
        for term in &self.children {
            write!(f, "{}", term)?
        }
        writeln!(f)?;
        Ok(())
    }
}

impl Header {
    #[inline]
    pub fn into_node(self, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: ASTKind::Header(Box::new(self)), range }
    }
}
