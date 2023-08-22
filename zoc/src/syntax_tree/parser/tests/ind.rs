use super::*;

#[test]
fn ind() {
    let src = r#"
(
ind

Set0

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

Set0

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
