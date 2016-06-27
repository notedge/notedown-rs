use crate::parse;

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

    # Title 1
    ## Title 2.1
    ## Title 2.2

    \toc_ignore

    ## Title 2.3
    ### Title 3.1
    ### Title 3.2
    ### Title 3.3
    "#;
    println!("{:#?}", parse(input).toc(9))
}

#[test]
fn test_math() {
    let input = r#"
    inline math: $x^2$

    display math: $$x^2$$

    block math:

    $$\frac{-b\pm\sqrt{b^2-4ac}}{2a}$$

    "#;
    println!("{:#?}", parse(input))
}
