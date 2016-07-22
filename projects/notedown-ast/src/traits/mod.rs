use std::fmt::Formatter;
use std::fmt;

mod display;
mod slugify;
mod html;



pub trait Slugify {
    fn slugify(&self) -> String;
}
