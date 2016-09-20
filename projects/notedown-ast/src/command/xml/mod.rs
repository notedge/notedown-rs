use super::*;
use crate::ASTNodes;

#[derive(Clone, Eq, PartialEq)]
pub enum XMLCommandKind {
    OpenClose {
        /// `|``<cmd`
        start: usize,
        /// `|``>`
        middle: usize,
        /// `|``</cmd>`
        end: usize,
    },
    /// ```md
    /// `<cmd arg=1/>`
    /// ```
    SelfClose {
        // <cmd
        start: usize,
        // </cmd>
        end: usize,
    },
}

/// ```md
/// <cmd[][] arg=1>body text</cmd>
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct XMLCommand {
    /// `cmd`
    pub cmd: String,
    pub kind: XMLCommandKind,
    pub pattern: CommandPattern,
    pub options: CommandOptions,
    pub body: ASTNodes,
}

impl XMLCommandKind {
    pub fn start_range(&self, name: &str) -> Range<usize> {
        match self {
            | Self::OpenClose { start, middle: _, end: _ } // \n
            | Self::SelfClose { start, end: _ } => {
                Range { start: *start, end: start + 1 + name.len() }
            }
        }
    }
    pub fn end_range(&self, name: &str) -> Range<usize> {
        match self {
            | Self::OpenClose { start: _, middle: _, end } // \n
            | Self::SelfClose { start: _, end } => {
                Range { start: *end, end: end + 1 + name.len() }
            }
        }
    }
}

impl XMLCommand {
    pub fn open_close(body: ASTNodes, literal: CommandPattern, options: CommandOptions) -> Self {
        let kind = XMLCommandKind::OpenClose { start: 0, middle: 0, end: 0 };
        Self { cmd: "".to_string(), kind, pattern: literal, options, body }
    }

    pub fn self_close(literal: CommandPattern, options: CommandOptions) -> Self {
        let kind = XMLCommandKind::SelfClose { start: 0, end: 0 };
        Self { cmd: "".to_string(), kind, pattern: literal, options, body: vec![] }
    }
}

impl XMLCommand {
    pub fn modify_name(&mut self, new: String) -> (Range<usize>, Range<usize>) {
        let out = (self.kind.start_range(&self.cmd), self.kind.end_range(&self.cmd));
        self.cmd = new;
        return out;
    }
}
