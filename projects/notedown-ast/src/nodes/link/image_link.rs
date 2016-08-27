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
    pub size: Option<(usize, usize)>,
    /// 0: left
    /// 1: center
    /// 2: right
    pub layout: Option<ImageLayout>,
}

impl ImageLink {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        SmartLink::Image(box self).into_node(range)
    }
    #[inline]
    pub fn set_alt(mut self, msg: impl Into<String>) -> Self {
        self.alt = Some(msg.into());
        return self;
    }
    #[inline]
    pub fn set_size(mut self, width: usize, height: usize) -> Self {
        self.size = Some((width, height));
        return self;
    }
    #[inline]
    pub fn set_layout(mut self, layout: ImageLayout) -> Self {
        self.layout = Some(layout);
        return self;
    }
}
