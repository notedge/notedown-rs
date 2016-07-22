use crate::{CodeHighlight, ASTNodes, ASTNode, ASTKind, StyledNode, StyledKind, MathNode, MathKind};
use std::fmt::{Debug, Formatter, Write, Arguments, write};
use crate::traits::HTMLConfig;
use std::fmt;
use crate::nodes::Header;

pub trait WriteHTML {
    fn write_html(&self, f: &mut HTMLFormatter) -> fmt::Result;
}

pub struct HTMLFormatter<'a> {
    math_renderer: Option<fn(&MathNode) -> String>,
    buffer: &'a mut (dyn Write + 'a),
}

impl Write for HTMLFormatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.buffer.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> fmt::Result {
        self.buffer.write_fmt(args)
    }
}


impl WriteHTML for ASTNodes {
    fn write_html(&self, f: &mut HTMLFormatter) -> fmt::Result {
        for child in self {
            child.write_html(f)?
        }
        Ok(())
    }
}

impl WriteHTML for ASTNode {
    fn write_html(&self, f: &mut HTMLFormatter) -> fmt::Result {
        self.value.write_html(f)
    }
}

// notice that html5 is compatible with xhtml, but not the other way around
// so please close self-closing tags manually
// eg: <hr> -> <hr/>
impl WriteHTML for ASTKind {
    fn write_html(&self, f: &mut HTMLFormatter) -> fmt::Result {
        match self {
            ASTKind::Statements(_) => unimplemented!(),
            ASTKind::Header(_) => unimplemented!(),
            ASTKind::HorizontalRule => write!(f, "<hr/>"),
            ASTKind::Paragraph(children) => {
                f.write_str("<p>")?;
                children.write_html(f);
                f.write_str("</p>")
            }
            ASTKind::CodeBlock(inner) => inner.write_html(),
            ASTKind::TableView(inner) => inner.to_html(),
            ASTKind::ListView(inner) => inner.to_html(),
            ASTKind::Normal(inner) => unimplemented!(),
            ASTKind::Raw(inner) => unimplemented!(),
            ASTKind::Code(inner) => unimplemented!(),
            ASTKind::Styled(inner) => inner.to_html(),
            ASTKind::Math(_) => unimplemented!(),
            ASTKind::Escaped(_) => unimplemented!(),
            ASTKind::Link(_) => unimplemented!(),
            ASTKind::Value(_) => unimplemented!(),
            ASTKind::Command(_) => unimplemented!(),
        }
    }
}

impl WriteHTML for StyledNode {
    fn write_html(&self, f: &mut HTMLFormatter) -> fmt::Result {
        match self.kind {
            StyledKind::Normal => { children.to_html(f) }
            StyledKind::Italic => write!(f, "<i>{}</i>", children.to_html(f)),
            StyledKind::Bold => write!(f, "<b>{}</b>", children.to_html(f)),
            StyledKind::Emphasis => write!("<em>{}</em>", children.to_html(f)),
            StyledKind::Underline => write!("<u>{}</u>", children.to_html(f)),
            StyledKind::Strikethrough => write!("<del>{}</del>", children.to_html(f)),
            StyledKind::Undercover => write!(r#"<span class="undercover">{}</span>"#, children.to_html(f)),
        }
        Ok(())
    }
}

impl WriteHTML for MathNode {
    fn write_html(&self, f: &mut HTMLFormatter) -> fmt::Result {
        if let Some(renderer) = f.math_renderer {
            return f.write_str(&renderer(self));
        }
        match self.get_kind() {
            MathKind::Inline => write!(f, r#"<span class="math">${}$</span>"#, inner),
            MathKind::Display => write!(f, r#"<span class="math">$\displaystyle{{{}}}$</span>"#, inner),
            MathKind::BlockInline => write!(f, r#"<span class="math">${}$</span>"#, inner),
            MathKind::BlockDisplay => write!(f, r#"<span class="math">$\displaystyle{{{}}}$</span>"#, inner),
        }
    }
}


impl WriteHTML for Header {
    fn write_html(&self, f: &mut HTMLFormatter) -> fmt::Result {
        format!("<h{0}>{1}</h{0}>", self.level, self.children.write_html())
    }
}

impl WriteHTML for CodeHighlight {
    fn write_html(&self) -> String {
        unimplemented!()
    }
}

impl WriteHTML for ListView<T> {
    fn write_html(&self) -> String {
        unimplemented!()
    }
}

impl WriteHTML for TableView<T> {
    fn write_html(&self) -> String {
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
