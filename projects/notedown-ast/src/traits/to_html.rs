use crate::{
    ast::{ASTKind, Header, ListView, TableView},
    traits::ToHTML,
    ASTNode, CodeBlock,
};
use std::fmt::Debug;

impl<T: ToHTML> ToHTML for Vec<T> {
    fn to_html(&self) -> String {
        let s: Vec<_> = self.iter().map(ToHTML::to_html).collect();
        s.join("")
    }
}

// notice that html5 is compatible with xhtml, but not the other way around
// so please close self-closing tags manually
// eg: <hr> -> <hr/>
impl<M: Debug> ToHTML for ASTNode<M> {
    fn to_html(&self) -> String {
        match &self.kind {
            ASTKind::None => String::new(),
            ASTKind::Statements(children) => children.to_html(),
            ASTKind::Header(inner) => inner.to_html(),
            ASTKind::HorizontalRule => format!("<hr/>"),
            ASTKind::Paragraph(children) => format!("<p>{}</p>", children.to_html()),
            ASTKind::CodeBlock(inner) => inner.to_html(),
            ASTKind::MathBlock(inner) => format!(r#"<p class="math">$${}$$</p>"#, inner),
            ASTKind::TableView(inner) => inner.to_html(),
            ASTKind::ListView(inner) => inner.to_html(),
            ASTKind::Normal(inner) => format!("{}", inner),
            ASTKind::Raw(inner) => format!("`{}`", inner),
            ASTKind::Code(inner) => format!("<pre>{}</pre>", inner),
            ASTKind::Italic(children) => format!("<i>{}</i>", children.to_html()),
            ASTKind::Bold(children) => format!("<b>{}</b>", children.to_html()),
            ASTKind::Emphasis(children) => format!("<em>{}</em>", children.to_html()),
            ASTKind::Underline(children) => format!("<u>{}</u>", children.to_html()),
            ASTKind::Strikethrough(children) => format!("<del>{}</del>", children.to_html()),
            ASTKind::Undercover(children) => format!(r#"<span class="undercover">{}</span>"#, children.to_html()),
            ASTKind::MathInline(inner) => format!(r#"<span class="math">${}$</span>"#, inner),
            ASTKind::MathDisplay(inner) => format!(r#"<span class="math">$\displaystyle{{{}}}$</span>"#, inner),
            ASTKind::Escaped(char) => format!("{}", char),
            _ => {
                println!("HTML unimplemented ASTKind::{:#?}", self.kind);
                unreachable!()
            }
        }
    }
}

impl<T: ToHTML> ToHTML for Header<T> {
    fn to_html(&self) -> String {
        format!("<h{0}>{1}</h{0}>", self.level, self.children.to_html())
    }
}

impl ToHTML for CodeBlock {
    fn to_html(&self) -> String {
        unimplemented!()
    }
}

impl<T> ToHTML for ListView<T> {
    fn to_html(&self) -> String {
        unimplemented!()
    }
}

impl<T> ToHTML for TableView<T> {
    fn to_html(&self) -> String {
        unimplemented!()
    }
}

#[allow(dead_code)]
pub fn build_th(input: &str, e: u8) -> String {
    match e {
        1 => format!(r#"<th align="left">{}</th>"#, input),
        2 => format!(r#"<th align="right">{}</th>"#, input),
        3 => format!(r#"<th align="center">{}</th>"#, input),
        _ => format!("<th>{}</th>", input),
    }
}

#[allow(dead_code)]
pub fn build_td(input: &str, e: u8) -> String {
    match e {
        1 => format!(r#"<td align="left">{}</td>"#, input),
        2 => format!(r#"<td align="right">{}</td>"#, input),
        3 => format!(r#"<td align="center">{}</td>"#, input),
        _ => format!("<td>{}</td>", input),
    }
}
