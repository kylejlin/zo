use crate::lexer::lex;
use crate::parser::*;

#[test]
fn deb_0() {
    let src = r#"0"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn deb_1() {
    let src = r#"1"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn deb_2() {
    let src = r#"2"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn app_no_args() {
    let src = r#"(0)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn app_1_arg() {
    let src = r#"(0 1)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn ind() {
    let src = r#"
(
    ind

    Type0

    "Nat"

    // Index types
    ()

    // Variant constructor defs
    (
        (() ())
        ((0) ())
    )
)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn ind_nonliteral_universe() {
    let src = r#"
(
    ind

    1

    "Nat"

    // Index types
    ()

    // Variant constructor defs
    (
        (() ())
        ((0) ())
    )
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}

#[test]
fn ind_nonliteral_name() {
    let src = r#"
(
    ind

    Type0

    2

    // Index types
    ()

    // Variant constructor defs
    (
        (() ())
        ((0) ())
    )
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}

#[test]
fn vcon() {
    let src = r#"
(
    vcon

    (
        ind

        Type0

        "Nat"

        // Index types
        ()

        (
            (() ())
            ((0) ())
        )
    )

    // Variant constructor index - THIS MUST BE A NUMBER LITERAL.
    0
)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn vcon_nonliteral_ind() {
    let src = r#"
(
    vcon

    1

    // Variant constructor index - THIS MUST BE A NUMBER LITERAL.
    0
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}

#[test]
fn vcon_nonliteral_vcon_index() {
    let src = r#"
(
    vcon

    (
        ind

        Type0

        "Nat"

        // Index types
        ()

        (
            (() ())
            ((0) ())
        )
    )

    (2 3)
)"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}
