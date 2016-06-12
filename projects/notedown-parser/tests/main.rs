#[macro_use]
extern crate quote;
extern crate pest_generator;
extern crate proc_macro;

mod html;
mod pre_build;
mod text;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
#[ignore]
fn gen_parser() {
    pre_build::gen_note_down();
}
