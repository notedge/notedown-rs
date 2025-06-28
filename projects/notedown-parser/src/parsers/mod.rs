//! Parser combinators for Notedown language

use crate::helpers::{ParseState, NotedownNode};

// Re-export the trait for convenience
pub use crate::helpers::NotedownNode;
use notedown_ast::nodes::{
    NotedownKind, NotedownNode as AstNode, NotedownNodes,
    TextSpan, Header, Delimiter, CodeNode, MathNode, StyleNode,
    QuoteBlock, ListView, TableView, SmartLink
};
use diagnostic_quick::{Span, FileID};
use notedown_error::NoteError;
use nyar_hir::helpers::{Location, Position};

pub mod text;
pub mod header;
pub mod paragraph;
pub mod code;
pub mod math;
pub mod list;
pub mod quote;
pub mod table;
pub mod link;
pub mod delimiter;
pub mod styled;
pub mod command;
pub mod value;

// Re-export parsers
pub use text::*;
pub use header::*;
pub use paragraph::*;
pub use code::*;
pub use math::*;
pub use list::*;
pub use quote::*;
pub use table::*;
pub use link::*;
pub use delimiter::*;
pub use styled::*;
pub use command::*;
pub use value::*;





/// Parse a complete notedown document
pub fn parse_document(state: &mut ParseState) -> Result<NotedownNodes, NoteError> {
    let mut nodes = Vec::new();
    
    while !state.is_at_end() {
        state.skip_ignored();
        if state.is_at_end() {
            break;
        }
        
        let node = parse_block_element(state)?;
        nodes.push(node);
    }
    
    Ok(nodes)
}

/// Parse a block-level element
pub fn parse_block_element(state: &mut ParseState) -> Result<AstNode, NoteError> {
    state.skip_ignored();
    
    // Try parsing different block elements in order of precedence
    if let Ok(node) = header::parse_header(state) {
        return Ok(node);
    }
    
    if let Ok(node) = delimiter::parse_horizontal_rule(state) {
        return Ok(node);
    }
    
    if let Ok(node) = code::parse_code_block(state) {
        return Ok(node);
    }
    
    if let Ok(node) = math::parse_math_block(state) {
        return Ok(node);
    }
    
    if let Ok(node) = quote::parse_quote_block(state) {
        return Ok(node);
    }
    
    if let Ok(node) = list::parse_list(state) {
        return Ok(node);
    }
    
    if let Ok(node) = table::parse_table(state) {
        return Ok(node);
    }
    
    // Default to paragraph
    paragraph::parse_paragraph(state)
}

/// Parse inline elements within a paragraph or other container
pub fn parse_inline_elements(state: &mut ParseState) -> Result<NotedownNodes, NoteError> {
    let mut nodes = Vec::new();
    
    while !state.is_at_end() && !is_block_boundary(state) {
        let node = parse_inline_element(state)?;
        nodes.push(node);
    }
    
    Ok(nodes)
}

/// Parse a single inline element
pub fn parse_inline_element(state: &mut ParseState) -> Result<AstNode, NoteError> {
    // Try parsing different inline elements
    if let Ok(node) = styled::parse_styled_text(state) {
        return Ok(node);
    }
    
    if let Ok(node) = link::parse_link(state) {
        return Ok(node);
    }
    
    if let Ok(node) = code::parse_inline_code(state) {
        return Ok(node);
    }
    
    if let Ok(node) = math::parse_inline_math(state) {
        return Ok(node);
    }
    
    if let Ok(node) = command::parse_command(state) {
        return Ok(node);
    }
    
    // Default to text
    text::parse_text(state)
}

/// Check if we're at a block boundary (start of new block element)
fn is_block_boundary(state: &ParseState) -> bool {
    let rest = state.rest();
    
    // Check for various block markers
    rest.starts_with("\n\n") ||
    rest.starts_with("# ") ||
    rest.starts_with("## ") ||
    rest.starts_with("### ") ||
    rest.starts_with("#### ") ||
    rest.starts_with("##### ") ||
    rest.starts_with("###### ") ||
    rest.starts_with("---") ||
    rest.starts_with("```") ||
    rest.starts_with("$$") ||
    rest.starts_with("> ") ||
    rest.starts_with("- ") ||
    rest.starts_with("* ") ||
    rest.starts_with("+ ") ||
    rest.chars().next().map_or(false, |c| c.is_ascii_digit() && rest.contains(". "))
}

/// Helper function to create a location from start position to current position
fn create_location(state: &ParseState, start: Position) -> Location {
    Location {
        start,
        end: state.position,
        file: state.file.clone(),
    }
}

/// Helper function to create a span from start position to current position
pub fn create_span(state: &ParseState, start: Position) -> Span {
    let start_offset = start.offset as usize;
    let end_offset = state.position.offset as usize;
    Span::new(start_offset..end_offset)
}

/// Helper function to create a file ID
pub fn create_file_id(state: &ParseState) -> FileID {
    FileID::new(state.file.clone())
}