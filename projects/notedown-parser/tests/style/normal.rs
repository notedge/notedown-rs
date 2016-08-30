use crate::check_ast;
use notedown_ast::WriteHTML;
use std::{fs::File, io::prelude::*};

#[test]
fn test_math() {
    let mut file = File::create("tests/style/part1.md").unwrap();
    let f = check_ast(include_str!("part1.note"));
    // println!("{:#?}", f);
    file.write_all(f.write_html().as_bytes()).unwrap();
}

#[test]
fn test_style() {
    let mut file = File::create("tests/style/style.md").unwrap();
    let f = check_ast(include_str!("style.note"));
    // println!("{:#?}", f);
    file.write_all(f.write_html().as_bytes()).unwrap();
}

#[test]
fn test_asterisk() {
    let mut file = File::create("tests/style/asterisk.md").unwrap();
    let f = check_ast(include_str!("asterisk.note"));
    // println!("{:#?}", f);
    file.write_all(f.write_html().as_bytes()).unwrap();
}

#[test]
fn test_command() {
    let mut file = File::create("tests/style/commands.md").unwrap();
    let f = check_ast(include_str!("commands.note"));
    // println!("{:#?}", f);
    file.write_all(f.write_html().as_bytes()).unwrap();
}

#[test]
fn test_component() {
    let mut file = File::create("tests/style/component.md").unwrap();
    let f = check_ast(include_str!("component.note"));
    // println!("{:#?}", f);
    file.write_all(f.write_html().as_bytes()).unwrap();
}
