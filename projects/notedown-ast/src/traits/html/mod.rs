use std::fmt::{Display, Formatter, Write};
use std::mem::take;
use html_ast::{HtmlElement};
use notedown_error::{NoteError, Validation};
use crate::ast::title::HeadingLevel;
use crate::hir::{HeadingNode, NotedownHIR, NotedownNode, ParagraphKind, ParagraphNode, TextPlainNode, TextStyleNode};
use crate::NoteGenerator;


pub struct HtmlBuilder {
    root: String,
}

impl Default for HtmlBuilder {
    fn default() -> Self {
        Self {
            root: "article".to_string(),
        }
    }
}

impl NoteGenerator for HtmlBuilder {
    type Output = HtmlElement;

    fn generate(&mut self, info: &NotedownHIR) -> Validation<Self::Output> {
        let mut errors = vec![];
        match info.as_html(self, &mut errors) {
            Ok(out) => {
                Validation::Success { value: out, diagnostics: errors }
            }
            Err(e) => Validation::Failure {
                fatal: e,
                diagnostics: errors,
            }
        }
    }
}

trait AsHtml {
    fn as_html(&self, config: &HtmlBuilder, errors: &mut Vec<NoteError>) -> Result<HtmlElement, NoteError>;
}


impl AsHtml for NotedownHIR {
    fn as_html(&self, config: &HtmlBuilder, errors: &mut Vec<NoteError>) -> Result<HtmlElement, NoteError> {
        let mut out = HtmlElement::new(&config.root);
        for node in &self.node {
            out.push_child(node.as_html(config, errors)?)
        }
        Ok(out)
    }
}

impl AsHtml for NotedownNode {
    fn as_html(&self, config: &HtmlBuilder, errors: &mut Vec<NoteError>) -> Result<HtmlElement, NoteError> {
        match self {
            NotedownNode::Heading(v) => {
                v.as_html(config, errors)
            }
            NotedownNode::Paragraph(v) => {
                v.as_html(config, errors)
            }
        }
    }
}

impl AsHtml for HeadingNode {
    fn as_html(&self, config: &HtmlBuilder, errors: &mut Vec<NoteError>) -> Result<HtmlElement, NoteError> {
        let mut out = HtmlElement::default();
        match self.level {
            HeadingLevel::Part => {}
            HeadingLevel::Chapter => {}
            HeadingLevel::Section => {}
            HeadingLevel::Article => {}
            HeadingLevel::Header1 => {
                out.set_tag("h1")
            }
            HeadingLevel::Header2 => {  out.set_tag("h2")  }
            HeadingLevel::Header3 => {  out.set_tag("h3")}
            HeadingLevel::Header4 => {  out.set_tag("h4")  }
            HeadingLevel::Header5 => { out.set_tag("h5")  }
            HeadingLevel::Header6 => {  out.set_tag("h6")  }
        }
        out.set_id(&self.id);
        for term in &self.terms.terms {
            match term {
                ParagraphKind::Plain(v) => {
                    out.push_child(v.text.clone())
                }
                ParagraphKind::Style(v) => {
                    out.push_child(v.as_html(config, errors)?)
                }
            }

        }

        Ok(out)
    }
}

impl AsHtml for ParagraphNode {
    fn as_html(&self, config: &HtmlBuilder, errors: &mut Vec<NoteError>) -> Result<HtmlElement, NoteError> {
        let mut out = HtmlElement::new("p");
        for term in &self.terms {
            match term {
                ParagraphKind::Plain(v) => {
                    out.push_child(v.text.clone())
                }
                ParagraphKind::Style(v) => {
                    out.push_child(v.as_html(config, errors)?)
                }
            }

        }
        Ok(out)
    }
}

impl AsHtml for TextStyleNode {
    fn as_html(&self, config: &HtmlBuilder, errors: &mut Vec<NoteError>) -> Result<HtmlElement, NoteError> {
        let mut out = if self.italic {
           HtmlElement::new("i")
        } else if self.bold {
            HtmlElement::new("b")
        } else if self.underline {
            HtmlElement::new("u")
        } else {
            HtmlElement::new("span")
        };
        for term in &self.text.terms {
            match term {
                ParagraphKind::Plain(v) => {
                    out.push_child(v.text.clone())
                }
                ParagraphKind::Style(v) => {
                    out.push_child(v.as_html(config, errors)?)
                }
            }

        }
        Ok(out)
    }
}