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

    // TODO: Delete after debugging.
    println!("XXX.start:{src}");

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

#[test]
fn add_n_zero() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0

ind Nat
    case zero
    case succ(_: Nat)
    return Set0

fun add(-a: Nat, b: Nat): Nat
    match a
    case zero:
        b
    case succ(a_pred):
        succ(add(a_pred, b))
    return1 Nat

fun add_n_zero(-n: Nat): Eq(Nat, n)(add(n, zero))
    match n
    case zero:
        refl(Nat, zero)
    case succ(n_pred):
        match add_n_zero(n_pred)
        case refl:
            refl(Nat, succ(n_pred))
        use [in_n_pred_out_add_n_pred_zero]
        return Eq(Nat, succ(n_pred))(succ(in_n_pred_out_add_n_pred_zero))
    use n_capp
    return1 Eq(Nat, n_capp)(add(n_capp, zero))

add_n_zero
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn add_n_succ_m() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0

ind Nat
    case zero
    case succ(_: Nat)
    return Set0

fun add(-a: Nat, b: Nat): Nat
    match a
    case zero:
        b
    case succ(a_pred):
        succ(add(a_pred, b))
    return1 Nat

fun add_n_succ_m(-n: Nat, m: Nat): Eq(Nat, succ(add(n, m)))(add(n, succ(m)))
    match n
    case zero:
        refl(Nat, succ(m))
    case succ(n_pred):
        // Goal: Eq(Nat, succ(succ(add(n_pred, m))))(succ(add(n_pred, succ(m))))
        match add_n_succ_m(n_pred, m)
        case refl:
            refl(Nat, succ(succ(add(n_pred, m))))
        use [in_succ_add_out_add_succ]
        return Eq(Nat, succ(succ(add(n_pred, m))))(succ(in_succ_add_out_add_succ))
    use n_capp
    return1 Eq(Nat, succ(add(n_capp, m)))(add(n_capp, succ(m)))

add_n_succ_m
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

#[test]
fn add_commutative() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0

ind Nat
    case zero
    case succ(_: Nat)
    return Set0

fun add(-a: Nat, b: Nat): Nat
    match a
    case zero:
        b
    case succ(a_pred):
        succ(add(a_pred, b))
    return1 Nat

fun add_n_zero(-n: Nat): Eq(Nat, n)(add(n, zero))
    match n
    case zero:
        refl(Nat, zero)
    case succ(n_pred):
        match add_n_zero(n_pred)
        case refl:
            refl(Nat, succ(n_pred))
        use [in_n_pred_out_add_n_pred_zero]
        return Eq(Nat, succ(n_pred))(succ(in_n_pred_out_add_n_pred_zero))
    use n_capp
    return1 Eq(Nat, n_capp)(add(n_capp, zero))

fun add_n_succ_m(-n: Nat, m: Nat): Eq(Nat, succ(add(n, m)))(add(n, succ(m)))
    match n
    case zero:
        refl(Nat, succ(m))
    case succ(n_pred):
        // Goal: Eq(Nat, succ(succ(add(n_pred, m))))(succ(add(n_pred, succ(m))))
        match add_n_succ_m(n_pred, m)
        case refl:
            refl(Nat, succ(succ(add(n_pred, m))))
        use [in_succ_add_out_add_succ]
        return Eq(Nat, succ(succ(add(n_pred, m))))(succ(in_succ_add_out_add_succ))
    use n_capp
    return1 Eq(Nat, succ(add(n_capp, m)))(add(n_capp, succ(m)))

// TODO: Delete after debugging.
ind TodoWrong
    case todo_wrong
    return Set0

// TODO: Rename `a` and `b` to `m` and `n`.
// This is not essential.
// However, it would be nice for the params
// to be consistent with the params
// in `add_n_zero` and `add_n_succ_m`.
afun add_commutative(-a: Nat, b: Nat): Eq(Nat, add(a, b))(add(b, a))
    match a
    case zero:
        match add_n_zero(b)
        case refl:
            // refl(Nat, b)
            todo_wrong
        use [in_b_out_add_b_zero]
        return Eq(Nat, b)(in_b_out_add_b_zero)
    case succ(a_pred):
        // Goal: Eq(Nat, succ(add(a_pred, b)))(add(b, succ(a_pred)))
        match add_n_succ_m(b, a_pred)
        case refl:
            // Goal: Eq(Nat, succ(add(a_pred, b)))(succ(add(b, a_pred)))
            match add_commutative(a_pred, b)
            case refl:
                // refl(Nat, succ(add(a_pred, b)))
                todo_wrong
            use [in_apred_b_out_b_apred]
            return Eq(Nat, succ(add(a_pred, b)))(succ(in_apred_b_out_b_apred))
        use [in_succ_add_out_add_succ]
        return Eq(Nat, succ(add(a_pred, b)))(in_succ_add_out_add_succ)
    use a_capp
    return1 Eq(Nat, add(a_capp, b))(add(b, a_capp))
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(zo.clone());

    insta::assert_display_snapshot!(PrettyPrint(&zo));
}
