use notedown_ast::{
    text::{title::HeadingNode, NotedownAST, NotedownTerm},
    IgnoreNode, ParagraphNode,
};
use notedown_error::{NoteError, ParseState, StopBecause};
use notedown_parser::{parse_file, NoteParser};
use std::path::Path;

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
fn test2() -> Result<(), NoteError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out = parse_file(here.join("tests/text_mode/test.note"))?;
    println!("{:#?}", out);
    Ok(())
}
