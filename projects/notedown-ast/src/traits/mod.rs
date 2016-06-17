mod slugify;
mod to_html;
mod to_string;
#[cfg(feature = "render")]
mod yew;
#[cfg(feature = "render")]
pub use self::yew::{Renderable, Html};

pub trait ToHTML {
    fn to_html(&self) -> String;
}
pub trait Slugify {
    fn slugify(&self) -> String;
}
