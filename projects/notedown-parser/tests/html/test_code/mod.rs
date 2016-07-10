use super::*;


#[test]
fn code_inline() {
    let input = r#"
    `code`

    ``code`code``
    "#;
    let output = r#"
    <p><pre>code</pre></p>
    <p><pre>code`code</pre></p>
    "#;
    trim_eq(input, output);
}

#[test]
fn test_code() {
    let input = r#"
    ```js
    let r = 0
    ```
    "#;
    let output = r#"
    \js[]{"body": "\n        let r = 0\n        "}
    "#;
    trim_eq(input, output);
}