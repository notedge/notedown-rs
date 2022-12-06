use super::*;

impl Display for CommandArguments {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for EscapedCommand {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for XMLCommand {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for XMLCommandMarks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenClose { .. } => f.write_str("OpenClose"),
            Self::SelfClose { .. } => f.write_str("SelfClose"),
        }
    }
}

impl Display for ExternalCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\external[{}][{}bytes]", self.cmd, self.data.len())
    }
}
