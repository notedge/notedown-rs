use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header {
    pub level: usize,
    pub children: Vec<AST>,
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}
