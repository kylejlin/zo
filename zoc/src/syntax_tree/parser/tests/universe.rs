use super::*;

#[test]
fn set0() {
    let src = r#"Set0"#;
    let tokens = lex(src).unwrap();
    let ost = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&ost);
}

#[test]
fn set1() {
    let src = r#"Set1"#;
    let tokens = lex(src).unwrap();
    let ost = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&ost);
}

#[test]
fn set42() {
    let src = r#"Set42"#;
    let tokens = lex(src).unwrap();
    let ost = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&ost);
}

#[test]
fn prop0() {
    let src = r#"Prop0"#;
    let tokens = lex(src).unwrap();
    let ost = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&ost);
}

#[test]
fn prop1() {
    let src = r#"Prop1"#;
    let tokens = lex(src).unwrap();
    let ost = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&ost);
}

#[test]
fn prop42() {
    let src = r#"Prop42"#;
    let tokens = lex(src).unwrap();
    let ost = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(&ost);
}
