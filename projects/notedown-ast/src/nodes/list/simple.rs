use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ListSimpleNode {
    pub ignore_global_list_style: bool,
    pub first_symbol: ListPrefixSymbol,
    pub children: Vec<ListItem>,
}

impl Default for ListSimpleNode {
    fn default() -> Self {
        Self { ignore_global_list_style: true, first_symbol: ListPrefixSymbol::Hyphen, children: vec![] }
    }
}

macro_rules! simple_node {
    (@ListSimpleNode => $name:ident=>($t1:ident, $t2:ident)) => {
        #[inline]
        pub fn $name(children: Vec<ListItem>) -> Self {
            Self { ignore_global_list_style: false, first_symbol: ListPrefixSymbol::$t2, children }
        }
    };
    (@ListView => $name:ident=>($t1:ident, $t2:ident)) => {
        #[inline]
        pub fn $name(children: Vec<ListItem>) -> Self {
            Self::$t1(box ListSimpleNode::$name(children))
        }
    };
    (@ASTKind => $name:ident=>($t1:ident, $t2:ident)) => {
        #[inline]
        pub fn $name(children: Vec<ListItem>) -> ASTNode {
            ListView::$name(children).into()
        }
    };
    ($($name:ident => ($t1:ident, $t2:ident)),+ $(,)?) => (
        impl ListSimpleNode { $(simple_node!(@ListSimpleNode => $name=>($t1, $t2));)+ }
        impl ListView { $(simple_node!(@ListView => $name=>($t1, $t2));)+ }
        impl ASTKind { $(simple_node!(@ASTKind => $name=>($t1, $t2));)+ }
    );
}

simple_node![
    quote_list     => (Quote,     Quote),
    ordered_list   => (Ordered,   Hyphen),
    orderless_list => (Orderless, Arabic),
];
