use crate::{pretty_print::PrettyPrinted, test_utils::*};

#[test]
fn add_2_3() {
    use crate::{
        syntax_tree::rch_cst_to_ast::RchCstToAstConverter,
        typecheck::{
            error::TypeError, LazySubstitutionContext, LazyTypeContext, Normalized, TypeChecker,
        },
    };

    let nat_def = (
        "<NAT>",
        r#"(ind Type0 "Nat" () (
(() ())
((0) ())
))"#,
    );
    let zero_def = ("<ZERO>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let add_two_three_src = substitute_with_compounding(
        [
            nat_def,
            zero_def,
            succ_def,
            (
                "<ADD>",
                "(fun 0 (<NAT> <NAT>) Type0
(
    match 2 <NAT>

    (
        (0 1)

        (1 (1 0 (<SUCC> 2)))
    )
))",
            ),
            ("<2>", "(<SUCC> (<SUCC> <ZERO>))"),
            ("<3>", "(<SUCC> <2>)"),
        ],
        r#"(<ADD> <2> <3>)"#,
    );

    // TODO: Fix this test
    let cst = parse_rch_cst_or_panic(&add_two_three_src);
    let err = TypeChecker::default()
        .get_type(
            cst,
            LazyTypeContext::Base(Normalized::empty_static()),
            LazySubstitutionContext::Base(&[]),
        )
        .unwrap_err();

    match err {
        TypeError::TypeMismatch {
            expr,
            expected_type,
            actual_type,
            ..
        } => {
            panic!(
                "******EXPR*******\n{}\n\n******EXPECTED TYPE*******\n{}\n\n******ACTUAL TYPE*******\n{}\n\n******SRC*******\n{add_two_three_src}\n\n******EXPR.SPAN*******\n{:?}\n\n",
                PrettyPrinted(&RchCstToAstConverter::default().convert(expr.clone())),
                PrettyPrinted(expected_type.raw()),
                PrettyPrinted(actual_type.raw()),
                expr.span(),
            );
        }

        err => panic!("Unexpected err: {err:?}"),
    }
}
