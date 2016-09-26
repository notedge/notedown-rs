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
/// [!description][source][link](options)
/// ```
#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct ImageLink {
    ///
    pub source: String,
    /// alt in html
    pub description: Option<String>,
    /// alt in html
    pub link: Option<String>,
    /// force to use caption ignore global setting
    pub force_caption: Option<bool>,
    /// force to use layout ignore global setting
    /// -1: left
    /// 0: center
    /// 1: right
    pub layout: Option<ImageLayout>,
    /// force to use layout ignore global setting
    pub size: Option<(usize, usize)>,
    pub options: Option<CommandOptions>,
}

impl ImageLink {
    #[inline]
    pub fn into_node(self, range: MaybeRanged) -> ASTNode {
        SmartLink::Image(box self).into_node(range)
    }
    #[inline]
    pub fn set_src(&mut self, msg: impl Into<String>) {
        self.source = msg.into();
    }
    #[inline]
    pub fn set_alt(&mut self, msg: impl Into<String>) {
        self.description = Some(msg.into());
    }
    #[inline]
    pub fn set_link(&mut self, msg: impl Into<String>) {
        self.link = Some(msg.into());
    }
    #[inline]
    pub fn set_size(&mut self, width: usize, height: usize) {
        self.size = Some((width, height));
    }
    #[inline]
    pub fn set_layout(&mut self, layout: ImageLayout) {
        self.layout = Some(layout);
    }

    pub fn set_options(&mut self, options: CommandOptions) -> Vec<NoteError> {
        let mut options = options;
        let mut errors = vec![];

        options.extract_string_key("src").map(|f| self.set_src(f));
        options.extract_string_key("source").map(|f| self.set_src(f));

        self.parse_layout(&mut options, &mut errors);

        options.extract_string_key("alt").map(|f| self.set_alt(f));
        options.extract_string_key("caption").map(|f| self.set_alt(f));
        options.extract_string_key("description").map(|f| self.set_alt(f));

        options.extract_bool_key("force_caption").map(|f| self.force_caption = Some(f));

        self.options = Some(options);
        return errors;
    }

    fn parse_layout(&mut self, options: &mut CommandOptions, errors: &mut Vec<NoteError>) {
        let value = match options.extract_key("layout") {
            None => return,
            Some(s) => s.value,
        };
        match value {
            Value::Integer(i) if i.is_negative() => self.layout = Some(ImageLayout::Left),
            Value::Integer(i) if i.is_zero() => self.layout = Some(ImageLayout::Center),
            Value::Integer(i) if i.is_zero() => self.layout = Some(ImageLayout::Right),
            Value::String(s) if s.to_ascii_lowercase().eq("left") => self.layout = Some(ImageLayout::Left),
            Value::String(s) if s.to_ascii_lowercase().eq("center") => self.layout = Some(ImageLayout::Center),
            Value::String(s) if s.to_ascii_lowercase().eq("right") => self.layout = Some(ImageLayout::Right),
            _ => errors.push(NoteError::runtime_error(format!("Unknown layout option {}", value))),
        }
    }
}
