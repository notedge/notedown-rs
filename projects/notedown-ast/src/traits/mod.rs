mod arith;
mod display;
mod html;
mod slugify;

pub use html::{HTMLRenderer, WriteHTML};

pub trait Slugify {
    fn slugify(&self) -> String;
}
