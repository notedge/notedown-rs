use super::*;

impl ASTKind {
    pub fn as_list_view(&self) -> Option<ListView> {
        match self {
            ASTKind::ListView(v) => Some((**v).to_owned()),
            _ => None,
        }
    }
    pub fn as_list_text(&self) -> Option<TextKind> {
        match self {
            ASTKind::TextSpan(v) => Some((**v).to_owned()),
            _ => None,
        }
    }
}
