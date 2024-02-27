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

fn assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(
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
fn comprehensive_syntax_check() {
    let src = include_str!("samples/comprehensive_syntax_check.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn stdlib() {
    let src = include_str!("samples/std.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn two() {
    let src = include_str!("samples/two.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn add_two_three() {
    let src = include_str!("samples/add_two_three.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn list() {
    let src = include_str!("samples/list.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn list_nil() {
    let src = include_str!("samples/list_nil.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn list_cons() {
    let src = include_str!("samples/list_cons.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn rev() {
    let src = include_str!("samples/rev.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn eq() {
    let src = include_str!("samples/eq.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn eq_refl() {
    let src = include_str!("samples/eq_refl.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn eq_commutative() {
    let src = include_str!("samples/eq_commutative.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn eq_transitive() {
    let src = include_str!("samples/eq_transitive.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn add_n_zero() {
    let src = include_str!("samples/add_n_zero.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn add_n_succ_m() {
    let src = include_str!("samples/add_n_succ_m.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn add_commutative() {
    let src = include_str!("samples/add_commutative.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn eq_implies_substitutable() {
    let src = include_str!("samples/eq_implies_substitutable.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn substitutable_implies_eq() {
    let src = include_str!("samples/substitutable_implies_eq.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}

#[test]
fn ill_typed_internal_def() {
    let src = include_str!("samples/ill_typed_internal_def.may");
    let converted_leaf =
        assert_expression_and_its_topright_defs_are_well_typed_under_empty_tcon(src);
    insta::assert_display_snapshot!(PrettyPrint(&converted_leaf));
}
