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

impl Debug for CommandPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.pts.iter().cloned().map(|s| s.value)).finish()
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
            w.field("pattern", &self.pattern);
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

impl Display for CommandPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in &self.pts {
            write!(f, "[{}]", i)?;
        }
        Ok(())
    }
}

impl Display for CommandOptions {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for NormalCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            NormalCommandKind::OneLine => {
                write!(f, "\\{}", self.cmd)?;
                write!(f, "{}", self.pattern)?;
                write!(f, "{}", self.options)?;
                write!(f, ": {}", self.body.value)
            }
            NormalCommandKind::MultiLine => {
                todo!()
            }
        }
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
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for XMLCommand {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
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
        write!(f, "\\external[{}][{}bytes]", self.cmd, self.data.len())
    }
}
