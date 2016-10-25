use super::*;

/// Supported math modes
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MathKind {
    /// Math surround by `$` inline
    Inline,
    /// Math surround by `$$` inline
    Display,
    /// Math surround by `$` blocked
    BlockInline,
    /// Math surround by `$$` blocked
    BlockDisplay,
}

/// Supported math backends
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MathBackend {
    ///
    LaTeX = 0,
    ///
    AsciiMath,
    ///
    MathML,
}

///
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MathNode {
    ///
    pub kind: MathKind,
    ///
    pub raw: String,
    ///
    pub format: MathBackend,
}

impl Default for MathBackend {
    fn default() -> Self {
        Self::LaTeX
    }
}

impl Default for MathNode {
    fn default() -> Self {
        Self { kind: MathKind::BlockDisplay, raw: String::new(), format: Default::default() }
    }
}

impl Display for MathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.surround(f)
    }
}

impl MathKind {
    /// TODO: doc
    pub fn surround_begin(&self) -> &'static str {
        match self {
            Self::Inline => "$",
            Self::Display => "$$",
            Self::BlockInline => "\n\n$",
            Self::BlockDisplay => "\n\n$$",
        }
    }
    /// TODO: doc
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
    /// TODO: doc
    pub fn surround(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.kind.surround_begin())?;
        f.write_str(&self.raw)?;
        f.write_str(self.kind.surround_end())?;
        Ok(())
    }
}

impl MathNode {
    /// Parse given format string
    /// do nothing if parse failed
    pub fn set_format(&mut self, s: &str) -> &mut Self {
        MathBackend::new(s).map(|f| self.format = f);
        self
    }
}

impl MathBackend {
    ///
    pub fn new(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "tex" | "latex" => Some(Self::LaTeX),
            "ascii" => Some(Self::AsciiMath),
            _ => None,
        }
    }
}

macro_rules! math_node {
    (@MathNode => $name:tt => $t:tt) => {
        /// Constructor of [`MathNode`]
        #[inline]
        pub fn $name(math: String) -> Self {
            Self { kind: MathKind::$t, raw: math, ..Default::default() }
        }
    };
    (@ASTKind => $name:tt => $t:tt) => {
        /// Constructor of [`MathNode`]
        #[inline]
        pub fn $name(math: impl Into<String>, range: MaybeRanged) -> ASTNode {
            MathNode::$name(math.into()).into_node(range)
        }
    };
    ($($name:tt => $t:tt),+ $(,)?) => (
        impl MathNode {$(math_node!(@MathNode => $name=>$t);)+}
        impl ASTKind {$(math_node!(@ASTKind => $name=>$t);)+}
    );
}

math_node![
    math_inline  => Inline,
    math_display => Display,
    math_block   => BlockDisplay,
];
