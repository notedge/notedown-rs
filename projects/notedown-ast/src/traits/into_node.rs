use crate::{nodes::*, ASTKind, ASTNode, Command, Value};

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

macro_rules! into_node {
    ($t:ty => (box, $variant:expr)) => {
    impl Into<ASTKind> for $t {
        #[inline]
        fn into(self) -> ASTKind { $variant(box self).into() }
    }
    impl Into<ASTNode> for $t {
        #[inline]
        fn into(self) -> ASTNode { $variant(box self).into_node(None) }
    }
    impl IntoASTNode for $t {
        #[inline]
        fn into_node(self, range: MaybeRanged) -> ASTNode {
            $variant(box self).into_node(range)
        }
    }
    };
    ($t:ty => (ref, $variant:expr)) => {
    impl Into<ASTKind> for $t {
        #[inline]
        fn into(self) -> ASTKind { $variant(self).into() }
    }
    impl Into<ASTNode> for $t {
        #[inline]
        fn into(self) -> ASTNode { $variant(self).into_node(None) }
    }
    impl IntoASTNode for $t {
        #[inline]
        fn into_node(self, range: MaybeRanged) -> ASTNode {
            $variant(self).into_node(range)
        }
    }
    };
    ($($t:ty => ($name:expr, $kind:ident)),+ $(,)?) => (
        $(into_node!($t=>($kind, $name));)+
    );
}

into_node![
    QuoteBlock    => (ASTKind::QuoteNode,     box),
    Header        => (ASTKind::Header,        box),
    Delimiter     => (ASTKind::Delimiter,     box),
    ListView      => (ASTKind::ListView,      ref),
    OrderedList   => (ListView::Ordered ,     box),
    OrderlessList => (ListView::Orderless ,   box),
    TableView     => (ASTKind::TableView,     ref),
    SimpleTable  =>  (TableView::SimpleTable, box),
    CodeNode      => (ASTKind::CodeNode,      box),
    MathNode      => (ASTKind::MathNode,      box),
    TextSpan      => (ASTKind::TextSpan,      box),
    StyleNode     => (ASTKind::StyledSpan,    box),
    Command       => (ASTKind::Command,       box),
    Value         => (ASTKind::Value,         box),
];
