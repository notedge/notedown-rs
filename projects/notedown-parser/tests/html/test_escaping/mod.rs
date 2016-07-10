use super::*;

#[test]
fn escape_special() {
    let input = r#"
     \\
     \s
     \r
     \n
    "#;
    let output = r#"
    <p>\</p>
    "#;
    trim_eq(input, output)
}