use super::*;

use zoc::pretty_print::PrettyPrint;

fn parse_or_panic(src: &str) -> mnode::Expr {
    let tokens = crate::lexer::lex(src).unwrap();
    crate::parser::parse(tokens).unwrap()
}

fn assert_expr_is_well_typed_under_empty_tcon(ast: znode::Expr) {
    use zoc::{
        eval::Normalized,
        pretty_print::PrettyUnwrap,
        syntax_tree::{lexer::lex as zo_lex, parser::parse as zo_parse},
        typecheck::{LazyTypeContext, TypeChecker},
    };

    let src = PrettyPrint(&ast).to_string();
    let tokens = zo_lex(&src).unwrap();
    let ost = zo_parse(tokens).unwrap();

    let empty = Normalized::<[_; 0]>::new();
    TypeChecker::default()
        .get_type(
            ost.into(),
            LazyTypeContext::Base(empty.as_ref().convert_ref()),
        )
        .pretty_unwrap();
}

#[test]
fn two() {
    let src = r#"
ind Nat
    case zero
    case succ(_: Nat)
    return Set0

succ(succ(zero))
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn add_two_three() {
    let src = r#"
ind Nat
    case zero
    case succ(_: Nat)
    return Set0

let _2 = succ(succ(zero))

let _3 = succ(_2)

fun add(-a: Nat, b: Nat): Nat
    match a
    case zero:
        b
    case succ(a_pred):
        succ(add(a_pred, b))
    return1 Nat

add(_2, _3)
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn list() {
    let src = r#"
ind(T: Set0) List
    case nil
    case cons(_: T, _: List)
    return Set0
List
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn list_nil() {
    let src = r#"
ind(T: Set0) List
    case nil
    case cons(_: T, _: List)
    return Set0
nil
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn list_cons() {
    let src = r#"
ind(T: Set0) List
    case nil
    case cons(_: T, _: List)
    return Set0
cons
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn rev() {
    let src = r#"
ind(T: Set0) List
    case nil
    case cons(_: T, _: List)
    return Set0

fun rev(T: Set0, xs: List(T)): List(T)
    fun helper(-xs: List(T), acc: List(T)): List(T)
        match xs
        case nil:
            acc
        case cons(x, xs):
            helper(xs, cons(T)(x, acc))
        return1 List(T)
    helper(xs, nil(T))

ind Abc
    case a
    case b
    case c
    return Set0

let acons = cons(Abc)
let anil = nil(Abc)

let a_b_c = acons(a, acons(b, acons(c, anil)))

rev(Abc, a_b_c)
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn eq() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0
Eq
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn eq_refl() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0
refl
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn eq_commutative() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0

afun(T: Set0, a: T, b: T, eq: Eq(T, a)(b)): Eq(T, b)(a)
    match eq
    case refl:
        refl(T, a)
    use [c] return Eq(T, c)(a)
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn eq_transitive() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0

afun(
    T: Set0,
    a: T,
    b: T,
    c: T,
    ab: Eq(T, a)(b),
    bc: Eq(T, b)(c) // TODO: Support trailing commas.
): Eq(T, a)(c)
    let f =
        match ab
        case refl:
            afun(ac: Eq(T, a)(c)): Eq(T, a)(c)
                ac
        use [ina_outb]
        return For(_: Eq(T, ina_outb)(c)) -> Eq(T, a)(c)
    f(bc)
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}
