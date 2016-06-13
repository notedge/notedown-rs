use notedown_parser::parse;

#[test]
fn test_header() {
    let input = r#"
    # Title 1
    ## Title 2
    ### Title 3
    #### Title 4
    ##### Title 5
    ###### Title 6
    ####### Title 7

    # Title **bold**
    "#;
    println!("{:#?}", parse(input).toc(9))
}
