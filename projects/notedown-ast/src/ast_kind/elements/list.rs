use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ListView {
    QuoteList { style: String, body: ASTNodes },
    OrderedList { head: usize, body: ASTNodes },
    OrderlessList { body: ASTNodes },
    Details { summary: ASTNodes, body: ASTNodes },
}
