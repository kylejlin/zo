use super::*;

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
