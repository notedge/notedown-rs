use crate::ASTNode;
use yew::prelude::*;
pub use yew::{Html, Renderable};
use yew_katex::KaTeX;

impl Renderable for ASTNode {
    fn render(&self) -> Html {
        match self {
            ASTNode::None => html! {},
            ASTNode::Statements(_) => unimplemented!(),
            ASTNode::Header { .. } => unimplemented!(),
            ASTNode::HorizontalRule { .. } => unimplemented!(),
            ASTNode::Paragraph { .. } => unimplemented!(),
            ASTNode::CodeBlock { .. } => unimplemented!(),
            ASTNode::MathBlock { inner, .. } => html! {<KaTeX math=inner inline=false/>},
            ASTNode::TableView { .. } => unimplemented!(),
            ASTNode::QuoteList { .. } => unimplemented!(),
            ASTNode::OrderedList { .. } => unimplemented!(),
            ASTNode::OrderlessList { .. } => unimplemented!(),
            ASTNode::Normal { .. } => unimplemented!(),
            ASTNode::Raw { .. } => unimplemented!(),
            ASTNode::Code { .. } => unimplemented!(),
            ASTNode::Italic { children, .. } => children.iter().map(|e| html! {<i>{e}</i>}).collect(),
            ASTNode::Bold { children, .. } => children.iter().map(|e| html! {<b>{e}</b>}).collect(),
            ASTNode::Emphasis { .. } => unimplemented!(),
            ASTNode::Underline { .. } => unimplemented!(),
            ASTNode::Strikethrough { .. } => unimplemented!(),
            ASTNode::Undercover { .. } => unimplemented!(),
            ASTNode::MathInline { inner, .. } => html! {<KaTeX math=inner inline=true/>},
            ASTNode::MathDisplay { inner, .. } => html! {<KaTeX math=inner inline=false/>},
            ASTNode::Link { .. } => unimplemented!(),
            ASTNode::Escaped { .. } => unimplemented!(),
            ASTNode::Command { .. } => unimplemented!(),
            ASTNode::String { .. } => unimplemented!(),
            ASTNode::Integer { .. } => unimplemented!(),
            ASTNode::Decimal { .. } => unimplemented!(),
            ASTNode::Boolean { .. } => unimplemented!(),
            ASTNode::Array { .. } => unimplemented!(),
        }
    }
}
