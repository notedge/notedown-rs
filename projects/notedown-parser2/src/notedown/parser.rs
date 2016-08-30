use crate::notedown::{Markdown, MarkdownInline, MarkdownText};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_while1},
    character::is_digit,
    combinator::{map, not},
    error::ErrorKind,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Err::Error,
    IResult,
};

pub fn parse_markdown(i: &str) -> IResult<&str, Vec<Markdown>> {
    many1(alt((
        map(parse_header, |e| Markdown::Heading(e.0, e.1)),
        map(parse_unordered_list, |e| Markdown::UnorderedList(e)),
        map(parse_ordered_list, |e| Markdown::OrderedList(e)),
        map(parse_code_block, |e| Markdown::Codeblock(e.to_string())),
        map(parse_markdown_text, |e| Markdown::Line(e)),
    )))(i)
}

fn parse_strong(i: &str) -> IResult<&str, &str> {
    delimited(tag("**"), is_not("**"), tag("**"))(i)
}

fn parse_em(i: &str) -> IResult<&str, &str> {
    delimited(tag("*"), is_not("*"), tag("*"))(i)
}

fn parse_inline_code(i: &str) -> IResult<&str, &str> {
    delimited(tag("`"), is_not("`"), tag("`"))(i)
}

fn parse_link(i: &str) -> IResult<&str, (&str, &str)> {
    pair(delimited(tag("["), is_not("]"), tag("]")), delimited(tag("("), is_not(")"), tag(")")))(i)
}

fn parse_image(i: &str) -> IResult<&str, (&str, &str)> {
    pair(delimited(tag("!["), is_not("]"), tag("]")), delimited(tag("("), is_not(")"), tag(")")))(i)
}

// we want to match many things that are not any of our specail tags
// but since we have no tools available to match and consume in the negative case (without regex)
// we need to match against our tags, then consume one char
// we repeat this until we run into one of our special characters
// then we join our array of characters into a String
fn parse_plaintext(i: &str) -> IResult<&str, String> {
    map(many1(preceded(not(alt((tag("*"), tag("`"), tag("["), tag("!["), tag("\n")))), take(1u8))), |vec| vec.join(""))(i)
}

fn parse_markdown_inline(i: &str) -> IResult<&str, MarkdownInline> {
    alt((
        map(parse_em, |s: &str| MarkdownInline::Italic(s.to_string())),
        map(parse_inline_code, |s: &str| MarkdownInline::InlineCode(s.to_string())),
        map(parse_strong, |s: &str| MarkdownInline::Bold(s.to_string())),
        map(parse_image, |(tag, url): (&str, &str)| MarkdownInline::Image(tag.to_string(), url.to_string())),
        map(parse_link, |(tag, url): (&str, &str)| MarkdownInline::Link(tag.to_string(), url.to_string())),
        map(parse_plaintext, |s| MarkdownInline::Plaintext(s)),
    ))(i)
}

fn parse_markdown_text(i: &str) -> IResult<&str, MarkdownText> {
    terminated(many0(parse_markdown_inline), tag("\n"))(i)
}

// this guy matches the literal character #
fn parse_header_tag(i: &str) -> IResult<&str, usize> {
    map(terminated(take_while1(|c| c == '#'), tag(" ")), |s: &str| s.len())(i)
}

// this combines a tuple of the header tag and the rest of the line
fn parse_header(i: &str) -> IResult<&str, (usize, MarkdownText)> {
    tuple((parse_header_tag, parse_markdown_text))(i)
}

fn parse_unordered_list_tag(i: &str) -> IResult<&str, &str> {
    terminated(tag("-"), tag(" "))(i)
}

fn parse_unordered_list_element(i: &str) -> IResult<&str, MarkdownText> {
    preceded(parse_unordered_list_tag, parse_markdown_text)(i)
}

fn parse_unordered_list(i: &str) -> IResult<&str, Vec<MarkdownText>> {
    many1(parse_unordered_list_element)(i)
}

fn parse_ordered_list_tag(i: &str) -> IResult<&str, &str> {
    terminated(terminated(take_while1(|d| is_digit(d as u8)), tag(".")), tag(" "))(i)
}

fn parse_ordered_list_element(i: &str) -> IResult<&str, MarkdownText> {
    preceded(parse_ordered_list_tag, parse_markdown_text)(i)
}

fn parse_ordered_list(i: &str) -> IResult<&str, Vec<MarkdownText>> {
    many1(parse_ordered_list_element)(i)
}

fn parse_code_block(i: &str) -> IResult<&str, &str> {
    delimited(tag("```"), is_not("```"), tag("```"))(i)
}
