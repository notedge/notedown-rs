#![feature(lazy_cell)]

mod atomic;
mod helpers;
pub mod notedown;
mod text;
mod traits;

pub use self::traits::{parse_file, NoteParser};
