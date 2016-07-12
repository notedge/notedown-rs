use crate::ast_kind::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Header<T> {
    pub level: usize,
    pub children: Vec<T>,
}

impl<T: Display> Display for Header<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", "#".repeat(self.level),)?;
        for term in &self.children {
            write!(f, "{}", term)?
        }
        writeln!(f)?;
        Ok(())
    }
}
