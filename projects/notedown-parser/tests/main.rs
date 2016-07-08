use notedown_parser::{ParserConfig, AST};

mod html;
mod text;

#[test]
fn ready() {
    println!("it, works!")
}

pub fn parse(s: &str) -> AST {
    let mut cfg = ParserConfig::default();
    cfg.parse(s).unwrap_or_default()
}
