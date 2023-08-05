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
