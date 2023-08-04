mod errors;
pub mod third_party;
// pub mod store;
#[cfg(feature = "url")]
pub use url::Url;

pub use errors::{NoteError, NoteErrorKind, Validation};

pub use pex::{
    helpers, ParseResult,
    ParseResult::{Pending, Stop},
    ParseState, Regex, StopBecause,
};
