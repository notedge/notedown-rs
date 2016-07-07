pub trait ToHTML {
    fn to_html(&self) -> String;
}
pub trait Slugify {
    fn slugify(&self) -> String;
}
