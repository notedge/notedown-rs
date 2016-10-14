use super::*;

mod detailed;
mod item;
mod ordered;
mod orderless;
mod prefix;

pub use self::{detailed::DetailedList, item::ListItem, ordered::OrderedList, orderless::OrderlessList, prefix::ListPrefixSymbol};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ListView {
    /// ## Ordered List
    /// ```note
    /// 1.1. part1
    /// 1.2. part2
    ///      part2
    /// 1.3. part3
    ///
    /// 1.4. part4
    /// ```
    Ordered(Box<OrderedList>),
    /// ## Orderless List
    /// ```note
    /// - part1
    /// - part2
    ///   part2
    /// - part3
    ///
    /// - part4
    /// ```
    Orderless(Box<OrderlessList>),

    Details(Box<DetailedList>),
}

impl From<ListView> for ASTNode {
    fn from(node: ListView) -> Self {
        Self { value: ASTKind::ListView(node), range: None }
    }
}

impl ListView {
    pub fn children(&self) -> &Vec<ListItem> {
        match self {
            Self::Ordered(v) => &v.children,
            Self::Orderless(v) => &v.children,
            Self::Details(v) => &v.body,
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<ListItem> {
        match self {
            Self::Ordered(v) => &mut v.children,
            Self::Orderless(v) => &mut v.children,
            Self::Details(v) => &mut v.body,
        }
    }
}

impl ASTKind {
    pub fn as_list_view(&self) -> Option<ListView> {
        match self {
            ASTKind::ListView(v) => Some(v.to_owned()),
            _ => None,
        }
    }
}

macro_rules! list_view {
    (@ASTKind => $name:ident) => {
        #[inline]
        pub fn $name(children: Vec<ListItem>) -> ASTNode {
            ListView::$name(children).into()
        }
    };
    ($($name:ident),+ $(,)?) => (
        impl ASTKind { $(list_view!(@ASTKind => $name);)+ }
    );
}

list_view![ordered_list, orderless_list];
