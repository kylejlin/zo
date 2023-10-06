use super::*;

#[test]
fn eq() {
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
fn list() {
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

#[test]
fn custom_zo_name() {
    let src = r#"
ind Foo "Empty"
    return Set0

Foo
"#;
    let tokens = lex(src).unwrap();
    let cst = parse(tokens).unwrap();
    insta::assert_debug_snapshot!(cst);
}

#[test]
fn non_universe_return_type() {
    let src = r#"
ind(T: Set0) List
    case nil
    case cons(car: T, cdr: List)
    return illegal

List(Nat)
"#;
    let tokens = lex(src).unwrap();
    let err = parse(tokens).unwrap_err();
    insta::assert_debug_snapshot!(err);
}

// TODO: Add more tests.
