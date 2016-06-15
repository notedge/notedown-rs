use crate::AST;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentAware {
    None,
    Text,
    Function,
    Math,
    Code,
}


impl AST {
    pub fn ask_element(&self, line: u32, column: u32) -> ContentAware {
        match self {
            AST::None => ContentAware::None,
            AST::Statements(v) => {
                for item in v {
                    let e = item.ask_element(line, column);
                    if e != ContentAware::None { return e; }
                }
                return ContentAware::None;
            }
            AST::Header { .. } => { unimplemented!() }
            AST::HorizontalRule { .. } => { unimplemented!() }
            AST::Paragraph { .. } => { unimplemented!() }
            AST::Highlight { .. } => { unimplemented!() }
            AST::MathBlock { .. } => ContentAware::Math,
            AST::TableView { .. } => { unimplemented!() }
            AST::QuoteList { .. } => { unimplemented!() }
            AST::OrderedList { .. } => { unimplemented!() }
            AST::OrderlessList { .. } => { unimplemented!() }
            AST::Normal { .. } => { unimplemented!() }
            AST::Raw { .. } => { unimplemented!() }
            AST::Code { .. } => ContentAware::Code,
            AST::Italic { .. } => { unimplemented!() }
            AST::Bold { .. } => { unimplemented!() }
            AST::Emphasis { .. } => { unimplemented!() }
            AST::Underline { .. } => { unimplemented!() }
            AST::Strikethrough { .. } => { unimplemented!() }
            AST::Undercover { .. } => { unimplemented!() }
            AST::MathInline { .. } => ContentAware::Math,
            AST::MathDisplay { .. } => ContentAware::Math,
            AST::Link { .. } => { unimplemented!() }
            AST::Escaped { .. } => { unimplemented!() }
            AST::Command { .. } => { unimplemented!() }
            AST::String { .. } => { unimplemented!() }
            AST::Integer { .. } => { unimplemented!() }
            AST::Decimal { .. } => { unimplemented!() }
            AST::Boolean { .. } => { unimplemented!() }
            AST::Array { .. } => { unimplemented!() }
        }
    }
}