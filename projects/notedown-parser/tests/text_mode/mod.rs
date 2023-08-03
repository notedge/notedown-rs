use notedown_ast::{
    text::{title::HeadingNode, TextModeNode, TextModeTerm},
    IgnoreNode, ParagraphNode,
};
use notedown_parser::NoteParser;
use pex::ParseState;

#[test]
fn test() {
    let id = IgnoreNode::parse(ParseState::new("\n\r\n "));
    // println!("{}", id);
    println!("{:#?}", id);
}

#[test]
fn test2() {
    let test = ParseState::new(include_str!("test.note"));
    let id = TextModeTerm::parse(test);
    // println!("{}", id);
    println!("{:#?}", id);
}

#[test]
fn test_title() {
    let id = HeadingNode::parse(ParseState::new("== title"));
    // println!("{}", id);
    println!("{:#?}", id);
}
