use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ListView {
    QuoteList {
        style: String,
        body: Vec<AST>,
    },
    OrderedList {
        head: usize,
        body: Vec<AST>,
    },
    OrderlessList {
        body: Vec<AST>,
    },
}

impl Display for ListView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}