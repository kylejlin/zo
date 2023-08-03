use super::*;

#[test]
fn ind_eq() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0

Eq(Nat, zero)(zero)
"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(cst);
}
