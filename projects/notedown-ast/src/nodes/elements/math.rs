use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MathKind {
    Inline,
    Display,
    BlockInline,
    BlockDisplay,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MathNode {
    kind: MathKind,
    raw: String,
    format: Option<String>,
}

impl Default for MathNode {
    fn default() -> Self {
        Self { kind: MathKind::BlockDisplay, raw: String::new(), format: None }
    }
}

impl Display for MathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.surround(f)
    }
}

impl MathKind {
    pub fn surround_begin(&self) -> &'static str {
        match self {
            Self::Inline => "$",
            Self::Display => "$$",
            Self::BlockInline => "\n\n$",
            Self::BlockDisplay => "\n\n$$",
        }
    }
    pub fn surround_end(&self) -> &'static str {
        match self {
            Self::Inline => "$",
            Self::Display => "$$",
            Self::BlockInline => "$\n\n",
            Self::BlockDisplay => "$$\n\n",
        }
    }
}

impl MathNode {
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::MathNode(box self), range }
    }
    pub fn surround(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.kind.surround_begin())?;
        f.write_str(&self.raw)?;
        f.write_str(self.kind.surround_end())?;
        Ok(())
    }
}

impl MathNode {
    pub fn get_text(&self) -> String {
        self.raw.to_owned()
    }
    pub fn get_format(&self) -> String {
        match &self.format {
            Some(s) => s.to_owned(),
            None => "LaTeX".to_string(),
        }
    }
    pub fn set_format(mut self, s: String) -> Self {
        self.format = Some(s);
        return self;
    }
    pub fn get_kind(&self) -> MathKind {
        self.kind
    }
    pub fn set_kind(mut self, kind: MathKind) -> Self {
        self.kind = kind;
        return self;
    }
}

macro_rules! math_node {
    ($name:tt => $t:tt) => {
        impl MathNode {
            #[inline]
            pub fn $name(math: String) -> Self {
                Self { kind: MathKind::$t, raw: math, ..Default::default() }
            }
        }

        impl ASTKind {
            #[inline]
            pub fn $name(math: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
                MathNode::$name(math.into()).into_node(range)
            }
        }
    };
    ($($name:tt => $t:tt),+ $(,)?) => (
        $(math_node!($name=>$t);)+
    );
}

math_node![
    math_block => BlockDisplay,
    math_inline => Inline,
    math_display => Display,
];
