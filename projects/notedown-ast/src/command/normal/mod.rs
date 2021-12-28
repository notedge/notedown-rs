use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NormalCommandKind {
    OneLine,
    MultiLine,
}

/// ```md
/// \cmd[][](): args
/// ```
/// ```md
/// \cmd(
///     arg = 1
/// )
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct NormalCommand {
    pub cmd: String,
    pub kind: NormalCommandKind,
    pub options: CommandArguments,
    pub pattern: LiteralPattern,
    pub body: Literal<String>,
}
