use super::*;

#[test]
fn match_() {
    let src = r#"
(
    match

    // Matchee
    0

    // Return arity
    3

    // Return val
    (for (500) 600)

    // Cases
    (
        (0 1)

        (3 400)

        (1 2)
    )
)"#;
    let tokens = lex(src).unwrap();
    let ost = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&ost);
}

#[test]
fn missing_return_arity() {
    let src = r#"
(
    match

    // Matchee
    0

    // Return val
    (for (500) 600)

    // Cases
    (
        (0 1)

        (3 400)

        (1 2)
    )
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}

#[test]
fn non_parenthesized_cases() {
    let src = r#"
(
    match

    // Matchee
    0

    // Return arity
    3

    // Return val
    (for (500) 600)

    // Unparenthesized case
    (0 1)
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}
