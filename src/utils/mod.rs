mod html_escape;
mod table_align;
mod text_align;
mod text_escape;

pub use html_escape::*;
pub use table_align::*;
pub use text_align::*;
pub use text_escape::*;


pub fn capitalize_first_letter(text: impl AsRef<str>) -> String {
    let text = text.as_ref();
    let mut c = text.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}