use crate::check_ast;

#[test]
fn test_math() {
    check_ast(include_str!("part1.note"), include_str!("part1.yaml"));
}

#[test]
fn test_style() {
    check_ast(include_str!("style.note"), include_str!("style.yaml"));
}

#[test]
fn test_asterisk() {
    check_ast(include_str!("asterisk.note"), include_str!("asterisk.yaml"));
}

#[test]
fn test_command() {
    check_ast(include_str!("commands.note"), include_str!("commands.yaml"));
}

#[test]
fn test_component() {
    check_ast(include_str!("component.note"), include_str!("component.yaml"));
}
