mod pre_build;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
#[ignore]
fn gen_parser() {
    pre_build::gen_note_down();
}
