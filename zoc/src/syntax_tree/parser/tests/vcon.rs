use super::*;

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
    
    0
    
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
