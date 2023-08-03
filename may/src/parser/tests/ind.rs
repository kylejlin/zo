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

#[test]
fn ind_list() {
    let src = r#"
ind(T: Set0) List
    case nil
    case cons(car: T, cdr: List)
    return Set0

List(Nat)
"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(cst);
}
