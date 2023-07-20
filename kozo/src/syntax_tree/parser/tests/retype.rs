use super::*;

#[test]
fn retype() {
    let src = r#"
(
    retype

    // Interm
    (100 200)

    // Intype
    (300 400)

    // Outtype
    (500 600)

    // Intype rewrites
    (L0 R2 R3)

    // Outtype rewrites
    (R1 L2 R3)
)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}
