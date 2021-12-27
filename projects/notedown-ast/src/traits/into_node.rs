use crate::{nodes::*, ASTKind, ASTKind::ListView, ASTNode, Command, Value};

/// Convert element into [`ASTNode`]
pub trait IntoASTNode {
    /// Convert element into [`ASTNode`] with position
    fn into_node(self, range: MaybeRanged) -> ASTNode;
}

impl Into<ASTNode> for ASTKind {
    #[inline]
    fn into(self) -> ASTNode {
        self.into_node(None)
    }
}

impl IntoASTNode for ASTKind {
    #[inline]
    fn into_node(self, range: MaybeRanged) -> ASTNode {
        ASTNode { value: self, range }
    }
}

macro_rules! into_node_level1 {
    ($t:ty => (box, $name:ident)) => {
    impl Into<ASTKind> for $t {
        #[inline]
        fn into(self) -> ASTKind { ASTKind::$name(box self) }
    }
    impl Into<ASTNode> for $t {
        #[inline]
        fn into(self) -> ASTNode { ASTKind::$name(box self).into_node(None) }
    }
    impl IntoASTNode for $t {
        #[inline]
        fn into_node(self, range: MaybeRanged) -> ASTNode {
            ASTKind::$name(box self).into_node(range)
        }
    }
    };
    ($t:ty => (ref, $name:ident)) => {
    impl Into<ASTKind> for $t {
        #[inline]
        fn into(self) -> ASTKind { ASTKind::$name(self) }
    }
    impl Into<ASTNode> for $t {
        #[inline]
        fn into(self) -> ASTNode { ASTKind::$name(self).into_node(None) }
    }
    impl IntoASTNode for $t {
        #[inline]
        fn into_node(self, range: MaybeRanged) -> ASTNode {
            ASTKind::$name(self).into_node(range)
        }
    }
    };
    ($($t:ty => ($name:ident, $kind:ident)),+ $(,)?) => (
        $(into_node_level1!($t=>($kind, $name));)+
    );
}

into_node_level1![
    QuoteBlock => (QuoteNode,  box),
    Header     => (Header,     box),
    Delimiter  => (Delimiter,  box),
    ListView   => (ListView,   ref),
    TableView  => (TableView,  ref),
    CodeNode   => (CodeNode,   box),
    MathNode   => (MathNode,   box),
    TextSpan   => (TextSpan,   box),
    StyleNode  => (StyledSpan, box),
    Command    => (Command,    box),
    Value      => (Value,      box),
];

impl IntoASTNode for OrderedList {
    fn into_node(self, range: MaybeRanged) -> ASTNode {
        ListView::Ordered(box self).into_node(range)
    }
}
impl IntoASTNode for OrderlessList {
    fn into_node(self, range: MaybeRanged) -> ASTNode {
        ListView::Orderless(box self).into_node(range)
    }
}

impl IntoASTNode for SimpleTable {
    fn into_node(self, range: MaybeRanged) -> ASTNode {
        TableView::SimpleTable(box self).into_node(range)
    }
}
