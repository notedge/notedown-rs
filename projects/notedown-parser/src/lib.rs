#![feature(lazy_cell)]

mod atomic;
mod helpers;
mod loader;
pub mod notedown;
mod text;
mod traits;

pub use self::traits::NoteParser;
