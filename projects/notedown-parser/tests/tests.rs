//! Integration tests for Notedown parser

use notedown_parser::helpers::{NotedownNode, ParseState};
use notedown_parser::parsers::*;
use notedown_ast::nodes::elements::text::TextSpan;
use notedown_ast::nodes::{Header, Paragraph, Delimiter, CodeNode, MathNode};
use notedown_ast::nodes::{StyleNode, SmartLink, ListView, QuoteNode, TableView, Command, Value};
use notedown_error::NoteError;

#[cfg(test)]
mod text_tests {
    use super::*;

    #[test]
    fn test_parse_normal_text() {
        let mut state = ParseState::new("Hello world");
        let result = TextSpan::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_escaped_char() {
        let mut state = ParseState::new("\\*escaped*");
        let result = parse_escaped_char(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_emoji() {
        let mut state = ParseState::new(":smile:");
        let result = parse_emoji(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_hard_break() {
        let mut state = ParseState::new("  \n");
        let result = parse_hard_break(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_soft_break() {
        let mut state = ParseState::new("\n");
        let result = parse_soft_break(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_checkbox() {
        let mut state = ParseState::new("[x] checked");
        let result = parse_checkbox(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("[ ] unchecked");
        let result = parse_checkbox(&mut state);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod header_tests {
    use super::*;

    #[test]
    fn test_parse_atx_header() {
        let mut state = ParseState::new("# Header 1");
        let result = Header::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("## Header 2");
        let result = Header::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("###### Header 6");
        let result = Header::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_setext_header() {
        let mut state = ParseState::new("Header 1\n========");
        let result = Header::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("Header 2\n--------");
        let result = Header::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_header() {
        let mut state = ParseState::new("####### Too many hashes");
        let result = Header::parse(&mut state);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod paragraph_tests {
    use super::*;

    #[test]
    fn test_parse_simple_paragraph() {
        let mut state = ParseState::new("This is a simple paragraph.");
        let result = Paragraph::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_multiline_paragraph() {
        let mut state = ParseState::new("This is a\nmultiline paragraph\nwith several lines.");
        let result = Paragraph::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_multiple_paragraphs() {
        let mut state = ParseState::new("First paragraph.\n\nSecond paragraph.");
        let result = parse_paragraphs(&mut state);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod delimiter_tests {
    use super::*;

    #[test]
    fn test_parse_horizontal_rule() {
        let mut state = ParseState::new("---");
        let result = Delimiter::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("***");
        let result = Delimiter::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("___");
        let result = Delimiter::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_page_break() {
        let mut state = ParseState::new("<<<");
        let result = Delimiter::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_thematic_break() {
        let mut state = ParseState::new("===");
        let result = Delimiter::parse(&mut state);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod code_tests {
    use super::*;

    #[test]
    fn test_parse_fenced_code_block() {
        let mut state = ParseState::new("```rust\nfn main() {\n    println!(\"Hello\");\n}\n```");
        let result = CodeNode::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_inline_code() {
        let mut state = ParseState::new("`let x = 42;`");
        let result = parse_inline_code(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_indented_code_block() {
        let mut state = ParseState::new("    fn main() {\n        println!(\"Hello\");\n    }");
        let result = parse_indented_code_block(&mut state);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod math_tests {
    use super::*;

    #[test]
    fn test_parse_inline_math() {
        let mut state = ParseState::new("$x^2 + y^2 = z^2$");
        let result = MathNode::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_math_block() {
        let mut state = ParseState::new("$$\n\\int_0^1 x^2 dx\n$$");
        let result = MathNode::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_math_environment() {
        let mut state = ParseState::new("\\begin{equation}\nx = y + z\n\\end{equation}");
        let result = parse_math_environment(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_bracketed_math() {
        let mut state = ParseState::new("[math]x^2[/math]");
        let result = parse_bracketed_math(&mut state);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod styled_tests {
    use super::*;

    #[test]
    fn test_parse_bold() {
        let mut state = ParseState::new("**bold text**");
        let result = StyleNode::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("__bold text__");
        let result = StyleNode::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_italic() {
        let mut state = ParseState::new("*italic text*");
        let result = StyleNode::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("_italic text_");
        let result = StyleNode::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_strikethrough() {
        let mut state = ParseState::new("~~strikethrough~~");
        let result = parse_strikethrough(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_underline() {
        let mut state = ParseState::new("<u>underlined</u>");
        let result = parse_underline(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_highlight() {
        let mut state = ParseState::new("==highlighted==");
        let result = parse_highlight(&mut state);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod link_tests {
    use super::*;

    #[test]
    fn test_parse_basic_link() {
        let mut state = ParseState::new("[text](https://example.com)");
        let result = SmartLink::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_reference_link() {
        let mut state = ParseState::new("[text][ref]");
        let result = parse_reference_link(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_link() {
        let mut state = ParseState::new("[unclosed link");
        let result = SmartLink::parse(&mut state);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    #[test]
    fn test_parse_unordered_list() {
        let mut state = ParseState::new("- Item 1");
        let result = ListView::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("* Item 1");
        let result = ListView::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("+ Item 1");
        let result = ListView::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ordered_list() {
        let mut state = ParseState::new("1. First item");
        let result = ListView::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("42. Numbered item");
        let result = ListView::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_list() {
        let mut state = ParseState::new("-No space after marker");
        let result = ListView::parse(&mut state);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod quote_tests {
    use super::*;

    #[test]
    fn test_parse_blockquote() {
        let mut state = ParseState::new("> This is a quote");
        let result = QuoteNode::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("> Multi-line\n> quote");
        let result = QuoteNode::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_callout() {
        let mut state = ParseState::new("> [!NOTE]\n> This is a note");
        let result = parse_callout(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("> [!WARNING]\n> This is a warning");
        let result = parse_callout(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_quote() {
        let mut state = ParseState::new(">");
        let result = QuoteNode::parse(&mut state);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;

    #[test]
    fn test_parse_markdown_table() {
        let mut state = ParseState::new("| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |");
        let result = TableView::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_grid_table() {
        let mut state = ParseState::new("+----------+----------+\n| Header 1 | Header 2 |\n+----------+----------+");
        let result = parse_grid_table(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_table_alignment() {
        let mut state = ParseState::new("| Left | Center | Right |\n|:-----|:------:|------:|");
        let result = parse_table_separator(&mut state);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn test_parse_directive() {
        let mut state = ParseState::new("\\command{arg}");
        let result = Command::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("\\command[option]{arg}");
        let result = Command::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_macro() {
        let mut state = ParseState::new("@macro(param1, param2)");
        let result = parse_macro(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("@simple_macro");
        let result = parse_macro(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_command() {
        let mut state = ParseState::new("\\{invalid}");
        let result = Command::parse(&mut state);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn test_parse_string_value() {
        let mut state = ParseState::new("\"hello world\"");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("'single quoted'");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_number_value() {
        let mut state = ParseState::new("42");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("3.14");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("-123");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_boolean_value() {
        let mut state = ParseState::new("true");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("false");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_null_value() {
        let mut state = ParseState::new("null");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_array_value() {
        let mut state = ParseState::new("[]");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
        
        let mut state = ParseState::new("[1, 2, 3]");
        let result = Value::parse(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_value() {
        let mut state = ParseState::new("invalid");
        let result = Value::parse(&mut state);
        assert!(result.is_err());
    }
}