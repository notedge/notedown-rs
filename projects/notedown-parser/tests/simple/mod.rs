use crate::check_ast;

#[test]
fn test_header() {
    let input = r#"\
# Title 1
## Title 2
### Title 3
#### Title 4
##### Title 5
###### Title 6
####### Title 7

# Title **bold**
    "#;
    check_ast(input, include_str!("header.yaml"));
}
#[test]
fn test_text() {
    let input = r#"\
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore
eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt
in culpa qui officia deserunt mollit anim id est laborum.
    "#;
    check_ast(input, include_str!("text.yaml"));
}

#[test]
fn test_style() {
    let input = r#"
    *斜体 Italic*

    **粗体 Bold**

    ***斜粗体 Bold-Italic***

    ~下划线 Underline~

    ~~删除线 Strikethrough~~

    ~~~数据删除 Undercover~~~

    `代码 code`

    ``code`escape``
    "#;
    check_ast(input, include_str!("style.yaml"));
}
#[test]
fn test_math() {
    let input = r#"
    $x$ $$x^2$$

    $$x^2$$ $x$

    $$x^2$$ $$x^2$$

    $$x^4$$
    "#;
    check_ast(input, include_str!("math.yaml"));
}
#[test]
fn test_function() {
    let input = r#"
    \title: Test
    \date: 2018-01-08
    \netease[song][22724727]
    \netease{"song", 28629058, autoplay = true}
    \link[https://github.com/nyar-lang/notedown-rs]
    "#;
    check_ast(input, include_str!("math.yaml"));
}

#[test]
fn test_table() {
    let input = r#"
    |name | age | level
    :-|-:
    aster | 17
    "#;
    check_ast(input, include_str!("math.yaml"));
}

#[test]
fn test_quote() {
    let input = r#"
    > - 1
    >   2
    > - 3
    "#;
    check_ast(input, include_str!("quote.yaml"));
}

#[test]
fn test_url() {
    let input = r#"
    https://www.zhihu.com/question/311834230/answer/595009063
    http://mathworld.wolfram.com/PrimeFormulas.style

    Page210
    http://read.pudn.com/downloads133/ebook/566944/%E9%AB%98%E6%95%88%E7%A8%8B%E5%BA%8F%E7%9A%84%E5%A5%A5%E7%A7%98.pdf
    ftp://www.m-hikari.com/ams/ams-2012/ams-73-76-2012/kaddouraAMS73-76-2012.pdf
    "#;
    check_ast(input, include_str!("url.yaml"));
}
