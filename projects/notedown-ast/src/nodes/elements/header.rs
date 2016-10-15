use crate::nodes::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Header {
    pub level: u8,
    pub id: Option<String>,
    pub children: Vec<ASTNode>,
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

#[allow(missing_docs)]
impl Header {
    #[inline]
    pub fn header(children: ASTNodes, level: u8, range: MaybeRanged) -> ASTNode {
        Header { level, id: None, children }.into_node(range)
    }
    #[inline]
    pub fn get_id(&self) -> Option<String> {
        self.id.to_owned()
    }
    #[inline]
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}
