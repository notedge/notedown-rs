use crate::{traits::ToHTML, AST};
use crate::ast::{ASTKind, Header};

impl ToHTML for Vec<AST> {
    fn to_html(&self) -> String {
        let s: Vec<_> = self.iter().map(ToHTML::to_html).collect();
        s.join("")
    }
}

impl ToHTML for AST {
    fn to_html(&self) -> String {
        match self {
            AST::Node { kind,children,.. } |
            AST::Leaf { kind,.. } => {
                match kind {
                    ASTKind::None => String::new(),
                    ASTKind::Statements {..} => v.to_html(),
                    ASTKind::Header {  0:inner,.. } => inner.to_html(),
                    ASTKind::HorizontalRule { .. } => unimplemented!(),
                    ASTKind::Paragraph { .. } => format!("<p>{}</p>", children.to_html()),
                    ASTKind::CodeBlock { .. } => unimplemented!(),
                    ASTKind::MathBlock { .. } => unimplemented!(),
                    ASTKind::TableView { .. } => unimplemented!(),
                    ASTKind::ListView { .. } => unimplemented!(),
                    ASTKind::Normal { 0:inner,.. } => inner.to_owned(),
                    ASTKind::Raw {  0:inner,.. } => format!("`{}`", inner),
                    ASTKind::Code { .. } => unimplemented!(),
                    ASTKind::Italic {  .. } => format!("<i>{}</i>", children.to_html()),
                    ASTKind::Bold {  .. } => format!("<b>{}</b>", children.to_html()),
                    ASTKind::Emphasis {  .. } => format!("<em>{}</em>", children.to_html()),
                    ASTKind::Underline {  .. } => format!("<u>{}</u>", children.to_html()),
                    ASTKind::Strikethrough {  .. } => format!("<del>{}</del>", children.to_html()),
                    ASTKind::Undercover { .. } => format!("<span class=\"undercover\">{}</span>", children.to_html()),
                    ASTKind::MathInline { .. } => unimplemented!(),
                    ASTKind::MathDisplay { .. } => unimplemented!(),
                    ASTKind::Link { .. } => unimplemented!(),
                    ASTKind::Escaped { .. } => unimplemented!(),
                    ASTKind::Command { .. } => unimplemented!(),
                    ASTKind::String { .. } => unimplemented!(),
                    ASTKind::Number { .. } => unimplemented!(),
                    ASTKind::Boolean { .. } => unimplemented!(),
                    ASTKind::Array { .. } => unimplemented!(),
                    ASTKind::Object => {}
                }
            }
        }
    }
}

impl ToHTML for Header {
    fn to_html(&self) -> String {
        format!("<h{0}>{1}</h{0}>", self.level, self.children.to_html())
    }
}

// impl ToHTML for AST {
// fn to_html(&self) -> String {
// match self {
// ASTKind::Statements(e) => {
// let mut text = String::new();
// for node in e {
// text += &node.to_html()
// }
// let trimmed: Vec<_> = text.lines().map(|s| s.trim()).collect();
// trimmed.join("\n")
// }
// ASTKind::Header { level, children:e,.. } => format!("<h{0}>{1}</h{0}>", level, e.to_html()),
//
//
// ASTKind::Table { head, align, terms, column } => {
// let align_iter = align.iter().chain(repeat(&align[align.len() - 1]));
// let thead = {
// let mut head = head.iter().chain(repeat(&ASTKind::Space));
// let mut align = align_iter.clone();
// let mut thead = String::new();
// for _ in 0..*column {
// let h = head.next().unwrap().to_html();
// let a = *align.next().unwrap();
// thead.push_str(&format!("{}", build_th(&h, a)))
// }
// format!("<thead>{}</thead>", thead)
// };
// let mut trs = vec![];
// for term in terms {
// let mut head = term.iter().chain(repeat(&ASTKind::Space));
// let mut align = align_iter.clone();
// let mut thead = String::new();
// for _ in 0..*column {
// let h = head.next().unwrap().to_html();
// let a = *align.next().unwrap();
// thead.push_str(&format!("{}", build_td(&h, a)))
// }
// trs.push(format!("<tr>{}</tr>", thead))
// }
// format!("<table>{}<tbody>{}</tbody></table>", thead, trs.join(""))
// }
// ASTKind::Quote { body, style } => {
// let quote = body.iter().map(|s| s.to_html()).collect::<Vec<String>>().join("");
// match style.as_str() {
// "info" | "danger" | "warning" | "success" => {
// format!("<blockquote class=\"fancyquote {}\">{}</blockquote>", style, quote)
// }
// _ => format!("<blockquote>{}</blockquote>", quote),
// }
// }
// ASTKind::Ordered(v) => {
// let quote = v.iter().map(|s| format!("<li>{}</li>", s.to_html())).collect::<Vec<String>>().join("");
// format!("<ol>{}</ol>", quote)
// }
// ASTKind::Orderless(v) => {
// let quote = v.iter().map(|s| format!("<li>{}</li>", s.to_html())).collect::<Vec<String>>().join("");
// format!("<ul>{}</ul>", quote)
// }
//
// ASTKind::Command(s, keys, values) => {
// let ref cfg = GLOBAL_CONFIG.lock().unwrap();
// format!("cmd: {}\narg: {:?}\nkvs: {:?}", s, keys, values)
// }
// /
// ASTKind::None => String::from(""),
// ASTKind::Newline => String::from("</br>"),
//
// ASTKind::Raw(s) => format!("<pre>{}</pre>", s),
// ASTKind::Code(s) => format!("<code>{}</code>", s),
//
// #[cfg(feature = "default")]
// ASTKind::MathInline(s) => format!("<span class=\"math\">${}$</span> ", s),
// #[cfg(feature = "desktop")]
// ASTKind::MathInline(s) => utils::rex_math_inline(s),
// #[cfg(feature = "default")]
// ASTKind::MathDisplay(s) => format!("<p class=\"math\">$${}$$</p> ", s),
// #[cfg(feature = "desktop")]
// ASTKind::MathDisplay(s) => utils::rex_math_display(s),
// _ => {
// let a = format!("HTML unimplemented ASTKind::{:?}", self);
// println!("{}", a.split("(").next().unwrap_or("Unknown"));
// format!("{:?}", self);
// unreachable!()
// }
// }
// }
// }

pub fn build_th(input: &str, e: u8) -> String {
    match e {
        1 => format!("<th align=\"left\">{}</th>", input),
        2 => format!("<th align=\"right\">{}</th>", input),
        3 => format!("<th align=\"center\">{}</th>", input),
        _ => format!("<th>{}</th>", input),
    }
}

pub fn build_td(input: &str, e: u8) -> String {
    match e {
        1 => format!("<td align=\"left\">{}</td>", input),
        2 => format!("<td align=\"right\">{}</td>", input),
        3 => format!("<td align=\"center\">{}</td>", input),
        _ => format!("<td>{}</td>", input),
    }
}
