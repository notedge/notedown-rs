use notedown_ast::ASTNode;
use notedown_parser::ParserConfig;

mod html;
mod text;

#[test]
fn ready() {
    println!("it, works!")
}

pub fn parse(s: &str) -> ASTNode<M> {
    let mut cfg = ParserConfig::default();
    cfg.parse(s).unwrap_or_default()
}
