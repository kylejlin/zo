use super::*;

#[test]
fn match_() {
    let src = r#"
(
    match

    // Matchee
    0

    // Econ extension length
    1

    // Return type
    3

    // Cases
    (
        (0 1)

        contra

        (1 2)
    )
)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn missing_return_type() {
    let src = r#"
(
    match

    // Matchee
    0

    // Econ extension length
    3

    // Cases
    (
        contra
    )
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}

#[test]
fn non_numeric_econ_extension_length() {
    let src = r#"
(
    match

    // Matchee
    0

    // Econ extension length
    (100 200)

    // Return type
    (0 1)

    // Cases
    (
        contra
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

    0

    3

    (100 200)

    (0 1)
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}
