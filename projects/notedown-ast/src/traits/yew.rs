use crate::AST;
use yew::prelude::*;
pub use yew::{Html, Renderable};
use yew_katex::KaTeX;

impl Renderable for AST {
    fn render(&self) -> Html {
        match self {
            AST::None => html! {},
            AST::Statements(_) => unimplemented!(),
            AST::Header { .. } => unimplemented!(),
            AST::HorizontalRule { .. } => unimplemented!(),
            AST::Paragraph { .. } => unimplemented!(),
            AST::CodeBlock { .. } => unimplemented!(),
            AST::MathBlock { inner, .. } => html! {<KaTeX math=inner inline=false/>},
            AST::TableView { .. } => unimplemented!(),
            AST::QuoteList { .. } => unimplemented!(),
            AST::OrderedList { .. } => unimplemented!(),
            AST::OrderlessList { .. } => unimplemented!(),
            AST::Normal { .. } => unimplemented!(),
            AST::Raw { .. } => unimplemented!(),
            AST::Code { .. } => unimplemented!(),
            AST::Italic { children, .. } => children.iter().map(|e| html! {<i>{e}</i>}).collect(),
            AST::Bold { children, .. } => children.iter().map(|e| html! {<b>{e}</b>}).collect(),
            AST::Emphasis { .. } => unimplemented!(),
            AST::Underline { .. } => unimplemented!(),
            AST::Strikethrough { .. } => unimplemented!(),
            AST::Undercover { .. } => unimplemented!(),
            AST::MathInline { inner, .. } => html! {<KaTeX math=inner inline=true/>},
            AST::MathDisplay { inner, .. } => html! {<KaTeX math=inner inline=false/>},
            AST::Link { .. } => unimplemented!(),
            AST::Escaped { .. } => unimplemented!(),
            AST::Command { .. } => unimplemented!(),
            AST::String { .. } => unimplemented!(),
            AST::Integer { .. } => unimplemented!(),
            AST::Decimal { .. } => unimplemented!(),
            AST::Boolean { .. } => unimplemented!(),
            AST::Array { .. } => unimplemented!(),
        }
    }
}
