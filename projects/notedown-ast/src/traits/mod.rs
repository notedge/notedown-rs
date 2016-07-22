mod display;
mod slugify;
mod html;

pub use html::{HTMLRenderer, WriteHTML};

pub trait Slugify {
    fn slugify(&self) -> String;
}
