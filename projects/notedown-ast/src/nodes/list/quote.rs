use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct QuoteList {
    pub custom_style: Option<String>,
    pub children: Vec<ListItem>,
}

impl ListView {
    #[inline]
    pub fn quote_list(children: Vec<ListItem>) -> Self {
        let list = QuoteList { custom_style: None, children };
        Self::Quote(box list)
    }
}
