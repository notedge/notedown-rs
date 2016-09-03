use super::*;

mod detailed;
mod item;
mod ordered;
mod orderless;
mod prefix;
mod quote;

pub use self::{
    detailed::DetailedList, item::ListItem, ordered::OrderedList, orderless::OrderlessList, prefix::ListPrefixSymbol, quote::QuoteList,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ListView {
    /// ## Quote List
    /// ```note
    /// > part1
    /// > part2
    ///   part2
    /// > part3
    ///
    /// > part4
    /// ```
    Quote(Box<QuoteList>),
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
