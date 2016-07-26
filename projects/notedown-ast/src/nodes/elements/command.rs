use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandKind {
    /// ```md
    /// \cmd: args
    /// ```
    Inline,
    /// ```md
    /// \cmd(
    ///     arg = 1
    /// )
    /// ```
    Normal,

    /// `[]`
    SmartLink,
    /// ````md
    /// ```cmd(arg=1)
    /// body text
    /// ```
    /// ````
    CodeBlock,
    /// ```md
    /// <cmd arg=1>body text</cmd>
    /// ```
    OpenClose,
    /// ```md
    /// `<cmd arg=1/>`
    /// ```
    SelfClose,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Command {
    pub cmd: String,
    pub kind: CommandKind,
    pub args: Vec<ASTNode>,
    pub kvs: Map<String, ASTNode>,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let a = self.args.iter().map(|v| format!("{}", v));
        let kv = self.kvs.iter().map(|(k, v)| format!("{} = {}", k, v));
        write!(f, "\\{}({})", self.cmd, a.chain(kv).collect::<Vec<_>>().join(", "))
    }
}

impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unimplemented!()
    }
}

impl Command {
    pub fn is(&self, rhs: impl AsRef<str>) -> bool {
        self.cmd.as_str() == rhs.as_ref()
    }
}
