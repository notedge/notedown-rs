use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderedList {
    pub ignore_global_list_style: bool,
    pub first_symbol: ListPrefixSymbol,
    pub children: Vec<ListItem>,
}

impl Default for OrderedList {
    fn default() -> Self {
        Self { ignore_global_list_style: true, first_symbol: ListPrefixSymbol::Hyphen, children: vec![] }
    }
}

impl ListView {
    #[inline]
    pub fn ordered_list(children: Vec<ListItem>) -> Self {
        let list = OrderedList { ignore_global_list_style: false, first_symbol: Default::default(), children };
        Self::Ordered(box list)
    }
}
