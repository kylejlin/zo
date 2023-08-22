use super::*;

#[test]
fn for_() {
    let src = r#"
(
    for

    // Param types
    (Set0 0 1)

    // Return type
    Prop0
)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}
