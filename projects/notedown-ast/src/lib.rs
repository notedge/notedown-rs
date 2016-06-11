mod ast;
mod traits;

pub use ast::{CommandKind, SmartLink, TextRange, Url, AST};

pub mod utils {
    pub use crate::traits::*;
    pub use text_utils::*;
    pub use url;
}
