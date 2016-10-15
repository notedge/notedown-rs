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

macro_rules! into_node_boxed {
    ($t:ty => $name:ident) => {
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
    ($($t:ty => $name:ident),+ $(,)?) => (
        $(into_node_boxed!($t=>$name);)+
    );
}

into_node_boxed![
    QuoteBlock => QuoteNode,
    Header     => Header,
    Delimiter  => Delimiter,
    CodeNode   => CodeNode ,
    MathNode   => MathNode,
    TextKind   => TextSpan,
    StyleNode  => StyledSpan,
    Command    => Command,
    Value      => Value,
];
