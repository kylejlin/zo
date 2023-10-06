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
fn val_is_let() {
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

#[test]
fn next_val_is_let() {
    let src = r#"
let three = succ(two)

let nine = mult(three, three)

add(nine, three)
"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(cst);
}

#[test]
fn no_next_val() {
    let src = r#"
let three = succ(two)
"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    assert_eq!(None, err);
}
