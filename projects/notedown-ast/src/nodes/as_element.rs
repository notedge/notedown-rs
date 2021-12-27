use super::*;

macro_rules! ast_view {
    ($f:ident => ($t1:ident,$t2:ident)) => {
    pub fn $f(&self) -> Option<$t1> {
        match self {
            ASTKind::$t2(v) => Some(v.to_owned()),
            _ => None,
        }
    }
    };
    ($f:ident => ($t1:ident,$t2:ident, box)) => {
    pub fn $f(&self) -> Option<$t1> {
        match self {
            ASTKind::$t2(v) => Some(v.as_ref().to_owned()),
            _ => None,
        }
    }
    };
    ($($f:ident => ($t1:ident,$t2:ident $(,box)?)),+ $(,)?) => (
        impl ASTKind { $(ast_view!($f=>($t1, $t2, box));)+ }
    );
}

ast_view![
    as_list_view  => (ListView, ListView ),
    as_table_view => (TableView, TableView),
    as_text_span => (TextSpan, TextSpan, box),
];
