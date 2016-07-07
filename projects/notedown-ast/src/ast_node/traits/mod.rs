

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