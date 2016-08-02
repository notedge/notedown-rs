use textwrap;

pub use textwrap::{dedent, fill};

pub fn indent(s: &str, space: usize) -> String {
    textwrap::indent(s, &" ".repeat(space))
}

pub fn indent_with(s: &str, prefix: &str) -> String {
    textwrap::indent(s, prefix)
}

/// https://stackoverflow.com/questions/60337455/how-to-trim-space-less-than-n-times
pub fn dedent_less_than(text: &str, max: usize) -> String {
    text.lines()
        .map(|line| {
            let mut max = max;
            line.chars()
                // Skip while `c` is a whitespace and at most `max` spaces
                .skip_while(|c| {
                    if max == 0 {
                        false
                    } else {
                        max -= 1;
                        c.is_whitespace()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
