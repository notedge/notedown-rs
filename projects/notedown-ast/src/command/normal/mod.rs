use super::*;

pub struct NormalCommandKind {}

/// ```md
/// \cmd[][](): args
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct NormalCommand {
    pub cmd: String,
    pub kind: NormalCommandKind,
    pub options: CommandOptions,
    pub pattern: CommandPattern,
    pub rest: Literal<String>,
}
