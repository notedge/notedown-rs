use notedown_ast::{
    ast::{HeadingSpan, ParagraphSpan},
    HtmlBuilder, NoteGenerator,
};
use notedown_error::{NoteError, ParseState};
use notedown_parser::{parse_file, NoteParser};
use std::{fs::File, io::Write, path::Path};

#[test]
fn test() {
    let id = ParagraphSpan::parse(ParseState::new(""));
    // println!("{}", id);
    println!("{:#?}", id);
}

#[test]
fn test_title() {
    let id = HeadingSpan::parse(ParseState::new("== title"));
    // println!("{}", id);
    println!("{:#?}", id);
}

#[test]
fn test2() -> Result<(), NoteError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = parse_file(here.join("tests/text_mode/test.note"))?;
    let mut output = File::create(here.join("tests/text_mode/test.html"))?;
    let mut html = HtmlBuilder::default();
    let html = html.generate(&input.as_hir()).unwrap();
    output.write_all(html.to_string().as_bytes())?;
    Ok(())
}
