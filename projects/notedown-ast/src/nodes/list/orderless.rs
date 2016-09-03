use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct OrderlessList {
    pub custom_style: Option<String>,
    pub children: Vec<ListItem>,
}

impl ListView {
    #[inline]
    pub fn orderless_list(children: Vec<ListItem>) -> Self {
        let list = OrderlessList { custom_style: None, children };
        Self::Orderless(box list)
    }
}
