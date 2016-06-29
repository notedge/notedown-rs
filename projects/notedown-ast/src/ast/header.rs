use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header {
    pub level: usize,
    pub children: Vec<ASTNode>,
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", "#".repeat(self.level),)?;
        for term in &self.children {
            write!(f, "{}", term)?
        }
        writeln!(f)?;
        Ok(())
    }
}
