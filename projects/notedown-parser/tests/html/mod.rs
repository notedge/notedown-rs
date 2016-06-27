use crate::parse;
use notedown_ast::utils::ToHTML;

mod easy;
mod normal;

pub fn trim_eq(input: &str, output: &str) {
    let input = parse(input).to_html().replace(" ", "").replace("\n", "");
    let output = output.replace(" ", "").replace("\n", "");
    assert_eq!(input, output);
}
