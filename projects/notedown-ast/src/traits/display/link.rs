use super::*;


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
            Self::Bare { link } => Display::fmt(link, f),
            Self::EMail { link } => Display::fmt(link, f),
            Self::Reciprocal { .. } => unimplemented!(),
            Self::TagBlock { .. } => unimplemented!(),
            Self::Reference { .. } => unimplemented!(),
            Self::Path { .. } => unimplemented!(),
            Self::PathWithText { .. } => unimplemented!(),
            Self::TagInline { .. } => unimplemented!(),
        }
    }
}
