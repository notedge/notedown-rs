#![allow(unused_must_use)]

use notedown_ast::Result;
use notedown_parser::NotedownParser;
mod normal;
mod simple;
mod style;
// mod text;

#[test]
fn ready() {
    println!("it, works!")
}

pub fn check_ast(source: &str, target: &str) -> Result<()> {
    let cfg = NotedownParser::default();
    let out = cfg.parse(source)?;
    assert_eq!(format!("{:#?}", out), target);
    Ok(())
}
