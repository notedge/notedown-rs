mod display;
mod slugify;
mod html;
mod arith;

pub use html::{HTMLRenderer, WriteHTML};

pub trait Slugify {
    fn slugify(&self) -> String;
}
