use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ImageLayout {
    Left,
    Center,
    Right,
}

/// ```note
/// [!! storage]
/// [./path/path.png : alt text ]
/// ```
#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct ImageLink {
    pub src: String,
    pub alt: Option<String>,
    /// 0: left
    /// 1: center
    /// 2: right
    pub layout: Option<ImageLayout>,
    pub size: Option<(usize, usize)>,
    pub options: Option<CommandOptions>,
}

impl ImageLink {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        SmartLink::Image(box self).into_node(range)
    }
    #[inline]
    pub fn set_alt(&mut self, msg: impl Into<String>) {
        self.alt = Some(msg.into());
    }
    #[inline]
    pub fn set_size(&mut self, width: usize, height: usize) {
        self.size = Some((width, height));
    }
    #[inline]
    pub fn set_layout(&mut self, layout: ImageLayout) {
        self.layout = Some(layout);
    }
    pub fn parse_options(mut self) -> Self {
        let options = match &mut self.options {
            None => return self,
            Some(s) => s,
        };
        options.extract_string_key("alt").map(|f| self.set_alt(f));
        return self;
    }
}
