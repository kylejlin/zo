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

fn assert_expr_is_ill_typed_under_empty_tcon(ast: znode::Expr) {
    use zoc::{
        eval::Normalized,
        pretty_print::PrettyUnwrapErr,
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
        .map(Normalized::into_raw)
        .pretty_unwrap_err();
}

fn assert_final_expression_and_topright_defs_are_well_typed_under_empty_tcon(
    src: &str,
) -> znode::Expr {
    let cst = parse_or_panic(src);
    let (converted_leaf, topright_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in topright_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    converted_leaf
}

#[test]
fn ill_typed_unused_def_does_not_affect_converted_leaf() {
    let src = r#"
ind Nat
    case zero
    case succ(_: Nat)
    return Set0

fun bad(n: Nat): Nat
    match n
    case zero:
        zero
    // missing `succ` case
    return1 Nat

succ(succ(zero))
"#;
    let cst = parse_or_panic(src);
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    // `substitutable_defs` should be `[Nat, zero, succ, bad]`.
    assert_eq!(4, substitutable_defs.len());

    // `Nat`, `zero`, and `succ` should be well typed.
    assert_expr_is_well_typed_under_empty_tcon(substitutable_defs[0].clone());
    assert_expr_is_well_typed_under_empty_tcon(substitutable_defs[1].clone());
    assert_expr_is_well_typed_under_empty_tcon(substitutable_defs[2].clone());

    // `bad` should be ill typed.
    assert_expr_is_ill_typed_under_empty_tcon(substitutable_defs[3].clone());

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn two() {
    let src = include_str!("samples/two.may");
    let converted_leaf =
        assert_final_expression_and_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    bc: Eq(T, b)(c),
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
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

afun add_commutative(-n: Nat, m: Nat): Eq(Nat, add(n, m))(add(m, n))
    match n
    case zero:
        match add_n_zero(m)
        case refl:
            refl(Nat, m)
        use [in_m_out_add_m_zero]
        return Eq(Nat, m)(in_m_out_add_m_zero)
    case succ(n_pred):
        // Goal: Eq(Nat, succ(add(n_pred, m)))(add(m, succ(n_pred)))
        match add_n_succ_m(m, n_pred)
        case refl:
            // Goal: Eq(Nat, succ(add(n_pred, m)))(succ(add(m, n_pred)))
            match add_commutative(n_pred, m)
            case refl:
                refl(Nat, succ(add(n_pred, m)))
            use [in_npred_m_out_m_apred]
            return Eq(Nat, succ(add(n_pred, m)))(succ(in_npred_m_out_m_apred))
        use [in_succ_add_out_add_succ]
        return Eq(Nat, succ(add(n_pred, m)))(in_succ_add_out_add_succ)
    use n_capp
    return1 Eq(Nat, add(n_capp, m))(add(m, n_capp))
"#;
    let cst = parse_or_panic(src);
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn eq_implies_substitutable() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0

afun eq_implies_substitutable(
    T: Set0,
    a: T,
    b: T,
    eq: Eq(T, a)(b),
    P: For(_: T) -> Prop0,
    pa: P(a),
): P(b)
    match eq
    case refl:
        pa
    use [in_a_out_b]
    return P(in_a_out_b)
"#;
    let cst = parse_or_panic(src);
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn substitutable_implies_eq() {
    let src = r#"
ind(T: Set0, left: T) Eq[_: T]
    case refl: [left]
    return Prop0

afun substitutable_implies_eq(
    T: Set0,
    a: T,
    b: T,
    sub: For(P: For(_: T) -> Prop0, pa: P(a)) -> P(b),
): Eq(T, a)(b)
    fun eq_a(c: T): Prop0
        Eq(T, a)(c)
    sub(eq_a, refl(T, a))
"#;
    let cst = parse_or_panic(src);
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn stdlib() {
    let src = include_str!("samples/std.may");
    let cst = parse_or_panic(src);
    let (converted_leaf, substitutable_defs) = may_to_zo(&cst).unwrap();

    assert_expr_is_well_typed_under_empty_tcon(converted_leaf.clone());

    for def in substitutable_defs {
        assert_expr_is_well_typed_under_empty_tcon(def);
    }

    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}
