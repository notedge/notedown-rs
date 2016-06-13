mod slugify;
mod to_html;
mod to_string;

pub trait ToHTML {
    fn to_html(&self) -> String;
}
pub trait Slugify {
    fn slugify(&self) -> String;
}
