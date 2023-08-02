use super::*;

#[test]
fn let_() {
    let src = r#"
let three = succ(two)

add(three, three)
"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(cst);
}

#[test]
fn let_val_is_let() {
    let src = r#"
let three =
    let two = succ(one)
    succ(two)

add(three, three)
"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(cst);
}
