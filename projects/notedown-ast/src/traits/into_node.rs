use crate::{nodes::*, ASTKind, ASTNode, Value};

/// Convert element into [`ASTNode`]
pub trait IntoASTNode {
    /// Convert element into [`ASTNode`] with position
    fn into_node(self, range: MaybeRanged) -> ASTNode;
}

macro_rules! into_node_boxed {
    ($t:ty => $name:ident) => {
        impl IntoASTNode for $t {
            #[inline]
            fn into_node(self, range: MaybeRanged) -> ASTNode {
                ASTNode { value: ASTKind::$name(box self), range }
            }
        }
    };
    ($($t:ty => $name:ident),+ $(,)?) => (
        $(into_node_boxed!($t=>$name);)+
    );
}

into_node_boxed![
    QuoteBlock => QuoteNode,
    Value      => Value,
];
