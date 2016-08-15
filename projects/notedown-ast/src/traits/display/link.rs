use super::*;
use crate::nodes::{EmailLink, HyperLink, ImageLink, TagReference, TwoWayLink};


impl Debug for SmartLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reference(v) => Debug::fmt(v, f),
            Self::Image(v) => Debug::fmt(v, f),
            Self::Normal(v) => Debug::fmt(v, f),
            Self::EMail(v) => Debug::fmt(v, f),
            Self::TwoWay(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for SmartLink {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Reference(v) => Display::fmt(v, f),
            Self::Image(v) => Display::fmt(v, f),
            Self::Normal(v) => Display::fmt(v, f),
            Self::EMail(v) => Display::fmt(v, f),
            Self::TwoWay(v) => Display::fmt(v, f),
        }
    }
}

impl Display for EmailLink {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for TagReference {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for ImageLink {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Debug for HyperLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       let w = &mut f.debug_struct("HyperLink");
        w.field("src", &self.src);
        w.field("is_bare", &self.is_bare);
        if let Some(s) = &self.text {
            w.field("text", &s);
        }
        if let Some(s) = &self.download {
            w.field("download", &s);
        }
        if let Some(s) = &self.target {
            w.field("target", &s);
        }
        w.finish()
    }
}

impl Display for HyperLink {
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
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for TwoWayLink {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
