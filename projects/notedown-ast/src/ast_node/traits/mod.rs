use crate::{
    traits::{Slugify, ToHTML},
    ASTNode,
};
use std::fmt::Debug;

impl<M: Debug> ToHTML for ASTNode<M> {
    fn to_html(&self) -> String {
        self.kind.to_html()
    }
}

impl<M> Slugify for ASTNode<M>
where
    ASTNode<M>: Debug,
{
    fn slugify(&self) -> String {
        self.kind.slugify()
    }
}
