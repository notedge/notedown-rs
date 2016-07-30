use std::fmt::{self, Display, Formatter};

/// 智能链接是指类似 `[ ]` 以及 `[[ ]]` 的结构
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SmartLink {
    /// ## Bare Link
    /// A link without the `[ ]`
    Bare { link: String },
    /// ## EMail
    /// ```note
    /// name@link.net
    /// ```
    EMail { link: String },
    /// ## Path Link
    /// A normal link without any description
    /// ```note
    /// [path-of-link]
    /// ```
    Path { path: String },
    /// ## Normal Link
    /// A normal link without any description
    /// ```note
    /// [alt-text: link]
    /// ```
    PathWithText { text: String, path: String },
    /// ## Two-way link
    /// This means this link will create a reference on the opposite side
    /// ```note
    /// [[link]]
    /// [[link > id]]
    /// ```
    Reciprocal { link: String, id: Option<String> },
    /// ## Tag Block
    /// ```note
    /// [^tag]: text text text
    /// [^tag]:
    ///     text text text
    ///     text text text
    /// ```
    TagBlock { tag: String, text: Vec<String> },
    /// ## Tag Inline
    /// Quote a number while defining
    /// ```note
    /// text [^tag: text text text] text
    /// ```
    TagInline { tag: String, text: Vec<String> },
    /// ## Tag Reference
    /// Quote a number, note that the name is irrelevant, and a number will always be generated in sequence at the end
    /// ```note
    /// text [^tag] text text text
    /// ```
    Reference { tag: String },
}

impl Display for SmartLink {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            // SmartLink::Path { path: from, to, alt, bind } => {
            //     let from_to = match to {
            //         None => format!("[{}]", from),
            //         Some(to) => format!("[{} > {}]", from, to),
            //     };
            //     match (bind, alt) {
            //         (None, None) => write!(f, "{}", from_to),
            //         (Some(bind), None) => write!(f, "{}[:{}]", from_to, bind),
            //         (None, Some(alt)) => write!(f, "{}[{}]", from_to, alt),
            //         (Some(bind), Some(alt)) => write!(f, "{}[{}:{}]", from_to, bind, alt),
            //     }
            // }
            // SmartLink::PathWithText { text: img, path: to, alt, bind } => {
            //     let img_to = match to {
            //         None => format!("[{}]", img),
            //         Some(s) => format!("[{} > {}]", img, s),
            //     };
            //     match (bind, alt) {
            //         (None, None) => write!(f, "{}", img_to),
            //         (Some(bind), None) => write!(f, "{}[:{}]", img_to, bind),
            //         (None, Some(alt)) => write!(f, "{}[{}]", img_to, alt),
            //         (Some(bind), Some(alt)) => write!(f, "{}[{}:{}]", img_to, bind, alt),
            //     }
            // }
            SmartLink::Bare { link } => Display::fmt(link, f),
            SmartLink::EMail { link } => Display::fmt(link, f),
            SmartLink::Reciprocal { .. } => unimplemented!(),
            SmartLink::TagBlock { .. } => unimplemented!(),
            SmartLink::Reference { .. } => unimplemented!(),
            SmartLink::Path { .. } => unimplemented!(),
            SmartLink::PathWithText { .. } => unimplemented!(),
            SmartLink::TagInline { .. } => unimplemented!(),
        }
    }
}
