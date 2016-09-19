mod show;
use super::*;
use std::fmt::{Debug, Write};

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal(v) => Debug::fmt(v, f),
            Self::Escaped(v) => Debug::fmt(v, f),
            Self::XML(v) => Debug::fmt(v, f),
            Self::External(v) => Debug::fmt(v, f),
        }
    }
}

impl Debug for NormalCommand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let a = self.args.values().map(|v| format!("{}", v));
        let kv = self.kvs.iter().map(|(k, v)| format!("{} = {}", k, v));
        write!(f, "\\{}({})", self.cmd, a.chain(kv).collect::<Vec<_>>().join(", "))
    }
}

impl Debug for XMLCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("Command");
        w.field("kind", &self.kind.to_string());
        if !self.literal.is_empty() {
            w.field("literal", &self.literal.iter().map(|f| f.value));
        }
        w.finish()
    }
}

impl Debug for ExternalCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("Command");
        w.field("kind", &"External");
        w.field("name", &self.cmd);
        w.field("size", &self.data.len());
        w.finish()
    }
}

impl Display for XMLCommandKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenClose { .. } => f.write_str("OpenClose"),
            Self::SelfClose { .. } => f.write_str("SelfClose"),
        }
    }
}

impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Normal(v) => v.hash(state),
            Self::Escaped(v) => v.hash(state),
            Self::XML(v) => v.hash(state),
            Self::External(v) => v.hash(state),
        }
    }
}

impl Hash for CommandOptions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.args.hash(state);
        for i in &self.kvs {
            i.hash(state)
        }
    }
}
