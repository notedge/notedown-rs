mod regroup;

use crate::{
    parser::regroup::{regroup_list_view, regroup_table_view},
    NotedownParser, Result,
};
use notedown_ast::{command::Command, nodes::CodeNode, ASTKind, ASTNode};
use notedown_pest::{NoteDownParser, Pair, Pairs, Parser, Rule};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl NotedownParser {
    pub fn parse(&self, input: &str) -> Result<ASTNode> {
        // let input = input.replace("\r\n", "\n").replace("\\\n", "").replace("\t", &" ".repeat(self.tab_size));
        let pairs = NoteDownParser::parse(Rule::program, &input)?;
        self.parse_program(pairs)
    }
    pub fn parse_program(&self, pairs: Pairs<Rule>) -> Result<ASTNode> {
        // let r = self.get_position(pairs.as_span());
        let mut codes = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::WHITE_SPACE => continue,
                Rule::LINE_SEPARATOR => continue,
                Rule::HorizontalRule => {
                    unimplemented!();
                    // AST::from("<hr/>")
                }
                Rule::Header => self.parse_header(pair),
                Rule::TextBlock => self.parse_paragraph(pair),
                Rule::List => {
                    codes.extend(self.parse_list(pair));
                    continue;
                }
                Rule::Table => {
                    codes.extend(self.parse_table(pair));
                    continue;
                }
                Rule::Code => self.parse_code_block(pair),
                Rule::CommandBlock => self.parse_command_block(pair),
                Rule::CommandLine => self.parse_command_line(pair),
                _ => debug_cases!(pair),
            };
            // println!("{:?}", code);
            codes.push(code);
        }

        // FIXME: fix range
        Ok(ASTKind::statements(codes, None))
    }
    fn parse_list(&self, pairs: Pair<Rule>) -> Vec<ASTNode> {
        // let r = self.get_position(pairs.as_span());
        let mut list_terms: Vec<(usize, &str, Vec<ASTNode>)> = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::LINE_SEPARATOR => continue,
                Rule::ListFirstLine | Rule::ListRestLine => {
                    let mut kind = "";
                    let mut indent = 0;
                    let mut inner = pair.into_inner();
                    while let Some(n) = inner.next() {
                        match n.as_rule() {
                            Rule::WHITE_SPACE => indent += 1,
                            Rule::ListMark | Rule::Vertical => {
                                kind = n.as_str();
                                break;
                            }
                            _ => debug_cases!(n),
                        }
                    }
                    let terms = inner.map(|pair| self.parse_span_term(pair)).collect();
                    list_terms.push((indent, kind, terms))
                }
                _ => debug_cases!(pair),
            };
        }
        return regroup_list_view(list_terms);
    }
    fn parse_table(&self, pairs: Pair<Rule>) -> Vec<ASTNode> {
        // let r = self.get_position(pairs.as_span());
        let mut table_terms = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::LINE_SEPARATOR => continue,
                Rule::TableFirstLine | Rule::TableRestLine => {
                    let mut line = vec![];
                    let mut inner = pair.into_inner();
                    let head = inner.next().unwrap();
                    let mut item = match head.as_rule() {
                        Rule::Vertical => vec![],
                        _ => vec![self.parse_span_term(head)],
                    };
                    for n in inner {
                        match n.as_rule() {
                            Rule::Vertical => {
                                line.push(item);
                                item = vec![]
                            }
                            _ => item.push(self.parse_span_term(n)),
                        }
                    }
                    if item.is_empty() {
                        table_terms.push(line)
                    }
                    else {
                        line.push(item);
                        table_terms.push(line)
                    }
                }
                _ => unreachable!(),
            };
        }
        return regroup_table_view(&table_terms);
    }
    pub fn parse_code_block(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pairs.as_span());
        let mut lang = String::new();
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITE_SPACE => continue,
                Rule::CodeLevel => continue,
                Rule::CodeMark => continue,
                Rule::SYMBOL => lang = pair.as_str().to_string(),
                Rule::CodeText => code = pair.as_str().to_string(),
                _ => debug_cases!(pair),
            };
        }
        CodeNode::code_block(lang, code).into_node(r)
    }

    fn parse_header(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pairs.as_span());
        let mut level = 0;
        let mut children = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITE_SPACE => continue,
                Rule::Sharp => level += 1,
                _ => children.push(self.parse_span_term(pair)),
            };
        }
        ASTKind::header(children, level, r)
    }
    pub fn parse_command_block(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pairs.as_span());
        let mut cmd = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Escape => continue,
                Rule::SYMBOL => cmd = pair.as_str().to_string(),
                _ => debug_cases!(pair),
            };
        }
        unimplemented!()
    }
    pub fn parse_command_line(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pairs.as_span());
        let mut cmd = String::new();
        let mut rest = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Escape | Rule::Colon => continue,
                Rule::SYMBOL => cmd = pair.as_str().to_string(),
                Rule::RestOfLine => rest = pair.as_str().to_string(),
                _ => debug_cases!(pair),
            };
        }
        ASTKind::command_line(cmd, rest, r)
    }
    pub fn parse_paragraph(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pairs.as_span());
        let codes = self.parse_span(pairs);
        if let 1 = codes.len() {
            match &codes[0].value {
                ASTKind::Statements(_) => {}
                ASTKind::Paragraph(_) => {}
                ASTKind::Delimiter(_) => {}
                ASTKind::Header(_) => {}
                ASTKind::TableView(_) => {}
                ASTKind::ListView(_) => {}
                ASTKind::CodeNode(_) => {}
                ASTKind::MathNode(_) => {}
                ASTKind::LinkNode(_) => {}
                ASTKind::TextSpan(_) => {}
                ASTKind::StyledSpan(_) => {}
                ASTKind::Command(_) => {}
                ASTKind::Value(_) => {}
            }
        }
        ASTKind::paragraph(codes, r)
    }
    fn parse_span(&self, pairs: Pair<Rule>) -> Vec<ASTNode> {
        pairs.into_inner().map(|pair| self.parse_span_term(pair)).collect()
    }
    fn parse_span_term(&self, pair: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pair.as_span());
        match pair.as_rule() {
            Rule::EOI => ASTNode::default(),
            Rule::Style => self.parse_styled_text(pair),
            Rule::TextRest => self.parse_normal_text(pair),
            Rule::TildeLine => self.parse_tilde_text(pair),
            Rule::Raw => self.parse_raw_text(pair),
            Rule::Math => self.parse_math_text(pair),
            Rule::RawRest | Rule::StyleRest | Rule::TildeRest | Rule::MathRest => self.parse_normal_text(pair),
            Rule::WHITE_SPACE | Rule::LINE_SEPARATOR => self.parse_normal_text(pair),
            Rule::Escaped => self.parse_escaped(pair),
            Rule::CommandBlock => self.parse_command_block(pair),
            Rule::URL => ASTKind::bare_link(pair.as_str(), r),
            _ => debug_cases!(pair),
        }
    }

    fn parse_normal_text(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pairs.as_span());
        ASTKind::text(pairs.as_str().to_string(), r)
    }
    fn parse_styled_text(&self, pairs: Pair<Rule>) -> ASTNode {
        let s = pairs.as_str().to_string();
        let r = self.get_position(pairs.as_span());
        let mut level = 0;
        let mut text = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Asterisk => continue,
                Rule::StyleLevel => level += pair.as_str().len(),
                Rule::StyleText => text.extend(self.parse_span(pair)),
                _ => debug_cases!(pair),
            };
        }
        match level {
            1 => ASTKind::emphasis(text, r),
            2 => ASTKind::strong(text, r),
            3 => ASTKind::italic_bold(text, r),
            _ => ASTKind::text(s, r),
        }
    }
    fn parse_tilde_text(&self, pairs: Pair<Rule>) -> ASTNode {
        let s = pairs.as_str().to_string();
        let r = self.get_position(pairs.as_span());
        let mut level = 0;
        let mut text = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Tilde => continue,
                Rule::TildeLevel => level += pair.as_str().len(),
                Rule::TildeText => text = self.parse_span(pair),
                _ => debug_cases!(pair),
            };
        }
        match level {
            1 => ASTKind::underline(text, r),
            2 => ASTKind::undercover(text, r),
            3 => ASTKind::undercover(text, r),
            _ => ASTKind::text(s, r),
        }
    }
    fn parse_raw_text(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pairs.as_span());
        for pair in pairs.into_inner() {
            if let Rule::RawText = pair.as_rule() {
                return ASTKind::text(pair.as_str().to_string(), r);
            };
        }
        return ASTNode::default();
    }
    fn parse_math_text(&self, pairs: Pair<Rule>) -> ASTNode {
        let s = pairs.as_str().to_string();
        let r = self.get_position(pairs.as_span());
        let mut inner = pairs.into_inner();
        let level = inner.next().unwrap().as_str().len();
        let text = inner.next().unwrap().as_str().to_string();
        match level {
            1 => ASTKind::math_inline(text, r),
            2 => ASTKind::math_display(text, r),
            _ => ASTKind::text(s, r),
        }
    }
    fn parse_escaped(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(pairs.as_span());
        ASTKind::escaped(pairs.as_str(), r)
    }
}
