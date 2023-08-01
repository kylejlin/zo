use super::*;

#[test]
fn match_() {
    let src = r#"
(
    match

    0

    3

    // Cases
    (
        (0 1)

        (3 400)

        (1 2)
    )
)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn non_parenthesized_cases() {
    let src = r#"
(
    match

    0

    3

    (0 1)
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}
