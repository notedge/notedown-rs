pub use notedown_ast::utils::*;
pub use notedown_pest::*;
pub use url::Url;

// pub fn unescape(s: &str, c: &str) -> String {
// let mut e = String::from("\\");
// e.push_str(c);
// s.replace(&e, c)
// }
//
// pub fn trim_escape(s: &str) -> &str {
// let s = s.trim();
// let mut out = s;
// if s.starts_with('\\') {
// out = &out[1..]
// }
// if s.ends_with('\\') {
// out = &out[..out.len() - 1]
// }
// return out;
// }
//
// pub fn trim_split_or(s: &str) -> Vec<String> {
// s.split(|c| c == ',' || c == '|').map(|s| s.trim().to_string()).collect()
// }
//
// pub fn str_escape(s: &str) -> String {
// match s.chars().next().unwrap() {
// '\'' => unescape(&s[1..s.len() - 1], "'"),
// '"' => unescape(&s[1..s.len() - 1], "\""),
// _ => s.to_string(),
// }
// }
//
// pub fn maybe_math(ctx: &mut ParserConfig, pair: Pair<Rule>) -> AST {
// let s = pair.as_str().trim();
// if s.starts_with("$$") && s.ends_with("$$") {
// let r = s[2..s.chars().count() - 2].to_string();
// if !r.contains("$$") {
// return AST::MathDisplay(r);
// }
// }
// let t = ctx.parse_text(s);
// match t {
// AST::None => AST::None,
// _ => AST::Paragraph(vec![t]),
// }
// }
//
// pub fn map_escape(c: &str) -> AST {
// match c {
// r"\r" => AST::None,
// r"\n" => AST::None,
// r"\t" => AST::Normal(String::from("    ")),
// _ => {
// unimplemented!()
// println!("escaping {:?}=>", c);
// AST::from(c.chars().last().unwrap())
// }
// }
// }
//
// pub fn map_white_space(c: &str) -> AST {
// match c {
// "\t" => AST::Normal(String::from("    ")),
// _ => AST::Normal(String::from(" ")),
// }
// }
