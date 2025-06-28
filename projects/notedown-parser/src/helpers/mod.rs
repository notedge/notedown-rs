//! Helper functions and utilities for Vampire parser

use notedown_error::NoteError;
use nyar_error::{ArcStr, NyarError};
use nyar_hir::helpers::{Location, Position};

/// Trait for nodes that can be parsed from input
pub trait NotedownNode: Sized {
    type Target;
    /// Parse this node type from input
    fn parse(state: &mut ParseState) -> Result<Self::Target, NoteError>;
}



/// Parse state for tracking position and file information during parsing
#[derive(Clone, Debug)]
pub struct ParseState {
    pub input: ArcStr,
    pub position: Position,
    pub file: ArcStr,
    pub indent_stack: Vec<usize>,
}

impl ParseState {
    pub fn new(input: String) -> Self {
        Self { 
            input: ArcStr::from(input), 
            position: Position::default(), 
            file: ArcStr::from("<unknown>"),
            indent_stack: vec![0],
        }
    }
    
    pub fn new_with_file(input: String, file: ArcStr) -> Self {
        Self { 
            input: ArcStr::from(input), 
            position: Position::default(), 
            file,
            indent_stack: vec![0],
        }
    }

    pub fn rest(&self) -> &str {
        unsafe { self.input.get_unchecked(self.position.offset as usize..) }
    }

    pub fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }
    
    pub fn peek_byte(&self) -> Option<u8> {
        self.rest().bytes().next()
    }
    
    pub fn starts_with(&self, pattern: &str) -> bool {
        self.rest().starts_with(pattern)
    }
    
    pub fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.position.advance(ch);
            Some(ch)
        } else {
            None
        }
    }
    
    pub fn advance_by(&mut self, text: &str) {
        self.position.advance_by(text);
    }

    pub fn is_at_end(&self) -> bool {
        self.position.offset as usize >= self.input.len()
    }

    pub fn current_location(&self) -> Location {
        Location { start: self.position, end: self.position, file: self.file.clone() }
    }

    /// Create a location spanning from start position to current position
    pub fn location_from(&self, start: Position) -> Location {
        Location { start, end: self.position, file: self.file.clone() }
    }

    /// Mark the current position for later span creation
    pub fn mark_position(&self) -> Position {
        self.position
    }

    /// Skip whitespace but preserve newlines for indentation tracking
    pub fn skip_whitespace_preserve_newlines(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    /// Skip all whitespace including newlines
    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip line comments starting with #
    pub fn skip_comment(&mut self) {
        if self.peek() == Some('#') {
            // Line comment - skip to end of line
            while let Some(ch) = self.peek() {
                self.advance();
                if ch == '\n' {
                    break;
                }
            }
        }
    }

    /// Skip both whitespace and comments
    pub fn skip_ignored(&mut self) {
        loop {
            let start_pos = self.position;
            self.skip_whitespace();
            self.skip_comment();
            // If position didn't change, we're done
            if self.position.offset == start_pos.offset {
                break;
            }
        }
    }

    /// Parse indentation level at start of line
    pub fn parse_indentation(&mut self) -> usize {
        let mut indent_level = 0;
        
        while let Some(ch) = self.peek() {
            match ch {
                ' ' => {
                    indent_level += 1;
                    self.advance();
                }
                '\t' => {
                    indent_level += 4; // Treat tab as 4 spaces
                    self.advance();
                }
                _ => break,
            }
        }
        
        indent_level
    }

    pub fn consume(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn consume_required(&mut self, expected: char) -> Result<(), NyarError> {
        if self.peek() == Some(expected) {
            self.advance();
            Ok(())
        } else {
            Err(NyarError::syntax_error(format!("Expected '{}', found {:?}", expected, self.peek())))
        }
    }

    pub fn consume_keyword(&mut self, keyword: &str) -> bool {
        self.skip_ignored();
        let start = self.position;

        for expected_char in keyword.chars() {
            if self.peek() == Some(expected_char) {
                self.advance();
            } else {
                // Restore position
                self.position = start;
                return false;
            }
        }

        // Check that the keyword is not part of a larger identifier
        if let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                // Restore position
                self.position = start;
                return false;
            }
        }

        true
    }

    /// Consume an operator (like ->, <=, >=, etc.) without identifier boundary checks
    pub fn consume_operator(&mut self, operator: &str) -> bool {
        self.skip_ignored();
        let start = self.position;

        for expected_char in operator.chars() {
            if self.peek() == Some(expected_char) {
                self.advance();
            } else {
                // Restore position
                self.position = start;
                return false;
            }
        }

        true
    }

    /// Parse an identifier
    pub fn parse_identifier(&mut self) -> Result<String, NyarError> {
        self.skip_ignored();
        
        if self.is_at_end() {
            return Err(NyarError::syntax_error("Unexpected end of input".to_string()));
        }
        
        let start_char = self.peek().unwrap();
        if !start_char.is_ascii_alphabetic() && start_char != '_' {
            return Err(NyarError::syntax_error("Invalid identifier start".to_string()));
        }
        
        let mut identifier = String::new();
        
        // Continue with letters, digits, or underscores
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        Ok(identifier)
    }

    /// Check if we're at the start of a new line (after newline)
    pub fn at_line_start(&self) -> bool {
        self.position.column == 1
    }

    /// Get current indentation level
    pub fn current_indent(&self) -> usize {
        *self.indent_stack.last().unwrap_or(&0)
    }

    /// Push new indentation level
    pub fn push_indent(&mut self, level: usize) {
        self.indent_stack.push(level);
    }

    /// Pop indentation level
    pub fn pop_indent(&mut self) -> Option<usize> {
        if self.indent_stack.len() > 1 {
            self.indent_stack.pop()
        } else {
            None
        }
    }
}

/// Main parser struct for Vampire language
pub struct VampireParser {
    pub(crate) file: ArcStr,
}

impl Default for VampireParser {
    fn default() -> Self {
        Self { file: ArcStr::from("<unknown>") }
    }
}

impl VampireParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_file(file: ArcStr) -> Self {
        Self { file }
    }

    pub fn set_file(&mut self, file: ArcStr) {
        self.file = file;
    }
}