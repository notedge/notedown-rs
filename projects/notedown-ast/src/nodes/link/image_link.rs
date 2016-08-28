use super::*;
use crate::NoteError;
use num::{Signed, Zero};

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
    pub source: String,
    /// alt in html
    pub description: Option<String>,
    pub force_caption: Option<bool>,
    /// -1: left
    /// 0: center
    /// 1: right
    pub layout: Option<ImageLayout>,
    pub size: Option<(usize, usize)>,
    pub link: Option<String>,
    pub options: Option<CommandOptions>,
}

impl ImageLink {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
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

    /// \begin{figure}[htp]
    // \centering % 图片居中
    // \includegraphics[width = 8.3cm]{figures/figure_1.png}
    // \caption{The caption of this figure.}
    // \label{fig:figure1label}
    // \end{figure}
    pub fn set_options(&mut self, options: CommandOptions) -> Vec<NoteError> {
        let mut errors = vec![];
        options.get_string_key("src").map(|f| self.set_src(f));
        options.get_string_key("source").map(|f| self.set_src(f));

        self.parse_layout(&options, &mut errors);

        options.get_string_key("alt").map(|f| self.set_alt(f));
        options.get_string_key("caption").map(|f| self.set_alt(f));
        options.get_string_key("description").map(|f| self.set_alt(f));
        self.options = Some(options.clone());
        return errors;
    }

    fn parse_layout(&mut self, options: &CommandOptions, errors: &mut Vec<NoteError>) {
        let value = match options.get_key("layout") {
            None => return,
            Some(s) => s.value.to_owned(),
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
