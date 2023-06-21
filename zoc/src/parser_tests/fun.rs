use super::*;

#[test]
fn nonrec_fun() {
    let src = r#"
(
    fun

    nonrec

    // Param types
    (3)

    // Return type
    4

    // Return value
    1
)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn rec_fun() {
    let src = r#"
(
    fun

    // Decreasing param index
    0

    // Param types
    (4)

    // Return type
    8

    1
)"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&cst);
}

#[test]
fn nonliteral_decreasing_index() {
    let src = r#"
    (
        fun
    
        (0 1)
    
        // Param types
        (4)
    
        // Return type
        8
    
        1
    )"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(&err);
}
