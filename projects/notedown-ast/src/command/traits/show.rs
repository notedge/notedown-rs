use super::*;
use crate::command::normal::NormalCommandKind;

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
        let w = &mut f.debug_struct("Command");
        w.field("kind", &self.kind.to_string());
        w.field("name", &self.cmd);
        w.field("pattern", &self.pattern);
        w.field("option", &self.options);
        w.field("body", &self.body);
        w.finish()
    }
}

impl Debug for EscapedCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("Command");
        w.field("kind", &"Escaped");
        w.field("name", &self.cmd);
        w.field("level", &self.level);
        w.field("pattern", &self.pattern);
        w.field("option", &self.options);
        w.field("body", &self.body);
        w.finish()
    }
}

impl Debug for XMLCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("Command");
        w.field("kind", &self.kind.to_string());
        if !self.pattern.is_empty() {
            w.field("literal", &self.pattern.iter().map(|f| f.value));
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

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal(v) => Display::fmt(v, f),
            Self::Escaped(v) => Display::fmt(v, f),
            Self::XML(v) => Display::fmt(v, f),
            Self::External(v) => Display::fmt(v, f),
        }
    }
}

impl Display for NormalCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for NormalCommandKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneLine => f.write_str("OneLine"),
            Self::MultiLine => f.write_str("MultiLine"),
        }
    }
}

impl Display for EscapedCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for XMLCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
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

impl Display for ExternalCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
