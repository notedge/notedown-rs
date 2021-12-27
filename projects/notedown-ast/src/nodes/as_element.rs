use super::*;

macro_rules! ast_view {
    ($f:ident => ($t1:ident,$t2:ident, ref)) => {
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
    ($($f:ident => ($t1:ident,$t2:ident, $t3:ident)),+ $(,)?) => (
        impl ASTKind { $(ast_view!($f=>($t1, $t2, $t3));)+ }
    );
}

ast_view![
    as_header     => (Header,    Header,     box),
    as_list_view  => (ListView,  ListView,   ref),
    as_table_view => (TableView, TableView,  ref),
    as_code       => (CodeNode,  CodeNode,   box),
    as_math       => (MathNode,  MathNode,   box),
    as_link       => (SmartLink, LinkNode,   ref),
    as_style_span => (StyleNode, StyledSpan, box),
    as_text_span  => (TextSpan,  TextSpan,   box),
];
