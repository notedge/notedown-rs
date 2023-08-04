use notedown_ast::{
    text::{title::HeadingNode, TextModeNode, TextModeTerm},
    IgnoreNode, ParagraphNode,
};
use notedown_parser::NoteParser;
use pex::{ParseState, StopBecause};

#[test]
fn test() {
    let id = ParagraphNode::parse(ParseState::new(""));
    // println!("{}", id);
    println!("{:#?}", id);
}

#[test]
fn test_title() {
    let id = HeadingNode::parse(ParseState::new("== title"));
    // println!("{}", id);
    println!("{:#?}", id);
}

#[test]
fn test2() -> Result<(), StopBecause> {
    let input = ParseState::new(include_str!("test.note"));
    let out = TextModeNode::parse(input);
    println!("{:#?}", out);
    Ok(())
}
