use yew::{Renderable, Html};
use notedown_parser::AST;

mod render;

impl Renderable for AST {
    fn render(&self) -> Html {
        unimplemented!()
    }
}