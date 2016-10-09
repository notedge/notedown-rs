mod escaped;
mod external;
mod normal;
mod options;
mod traits;
mod xml;

pub use self::xml::{XMLCommand, XMLCommandKind};
use crate::{
    command::{escaped::EscapedCommand, external::ExternalCommand, normal::NormalCommand},
    nodes::{Literal, MaybeRanged},
    value::*,
    ASTKind, ASTNode, Value,
};
use num::BigUint;
use std::ops::Range;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandOptions {
    pub args: SparseArray,
    pub kvs: OrderedMap,
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct CommandPattern {
    pts: Vec<Literal<String>>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Command {
    /// ```md
    /// \cmd: args
    /// ```
    Normal(NormalCommand),
    Escaped(EscapedCommand),
    XML(XMLCommand),
    External(ExternalCommand),
}

impl Command {
    #[inline]
    pub fn is(&self, rhs: impl AsRef<str>) -> bool {
        self.command().eq(rhs.as_ref())
    }
    #[inline]
    pub fn command(&self) -> &str {
        match self {
            Self::Normal(v) => v.cmd.as_str(),
            Self::Escaped(v) => v.cmd.as_str(),
            Self::XML(v) => v.cmd.as_str(),
            Self::External(v) => v.cmd.as_str(),
        }
    }
    #[inline]
    pub fn into_node(self, range: MaybeRanged) -> ASTNode {
        ASTNode { value: ASTKind::Command(box self), range }
    }
}
