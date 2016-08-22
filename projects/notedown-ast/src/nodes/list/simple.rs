use super::*;
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ListSimpleNode {
    pub ignore_global_list_style: bool,
    pub first_symbol: ListPrefixSymbol,
    pub children: Vec<Literal<ListItem>>,
}

impl Default for ListSimpleNode {
    fn default() -> Self {
        Self { ignore_global_list_style: true, first_symbol: ListPrefixSymbol::Hyphen, children: vec![] }
    }
}
