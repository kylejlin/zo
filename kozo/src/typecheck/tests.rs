use crate::{pretty_print::PrettyPrinted, test_utils::*};

#[test]
fn add_2_3() {
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
                "(fun 0 (<NAT> <NAT>) <NAT>
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

    let type_ = get_type_under_empty_tcon_and_scon_or_panic(&add_two_three_src);

    insta::assert_display_snapshot!(PrettyPrinted(type_.raw()));
}

#[ignore]
#[test]
fn rev_1_2_3() {
    let nat_def = (
        "<NAT>",
        r#"(ind Type0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let zero_def = ("<ZERO>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let one_def = ("<1>", "(<SUCC> <ZERO>)");
    let two_def = ("<2>", "(<SUCC> <1>)");
    let three_def = ("<3>", "(<SUCC> <2>)");
    let list_0_def = (
        "<LIST_0>",
        r#"(
            ind
    
            Type0
    
            "List"
    
            ()
    
            (
                // DB index stack is
                // 0 =>  List(T)
                // 1 => List 
                // 2 => T
    
                // nil
                (() ())
    
                // cons
                ((
                    2
    
                    // DB index stack is
                    // 0 => car
                    // 1 => List(T)
                    // 2 => List
                    // 3 => T
                    1
                ) ())
            )
        )"#,
    );
    let polymorphic_list_def = (
        "<POLYMORPHIC_LIST>",
        r#"(
    fun

    nonrec

    (Type0)

    Type0

    <LIST_0>
)"#,
    );
    let polymorphic_nil_def = (
        "<POLYMORPHIC_NIL>",
        r#"(
    fun

    nonrec

    (Type0)

    Type0

    (vcon <LIST_0> 0)
)"#,
    );
    let polymorphic_cons_def = (
        "<POLYMORPHIC_CONS>",
        r#"(
    fun

    nonrec

    (Type0)

    Type0

    (vcon <LIST_0> 1)
)"#,
    );
    let nat_nil_def = ("<NAT_NIL>", "(<POLYMORPHIC_NIL> <NAT>)");
    let cons_def = ("<NAT_CONS>", "(<POLYMORPHIC_CONS> <NAT>)");
    let normalized_nat_list_def = (
        "<NORMALIZED_NAT_LIST>",
        r#"(
            ind
    
            Type0
    
            "List"
    
            ()
    
            (
                // DB index stack is
                // 0 =>  List(Nat)
                // 1 => List 
    
                // nil
                (() ())
    
                // cons
                ((
                    <NAT>
    
                    // DB index stack is
                    // 0 => car
                    // 1 => List(Nat)
                    1
                ) ())
            )
        )"#,
    );
    let normalized_nat_nil_def = ("<NORMALIZED_NAT_NIL>", "(vcon <NORMALIZED_NAT_LIST> 0)");
    let normalized_nat_cons_def = ("<NORMALIZED_NAT_CONS>", "(vcon <NORMALIZED_NAT_LIST> 1)");
    let one_two_three_src = (
        "<123>",
        "(<NAT_CONS> <1> (<NAT_CONS> <2> (<NAT_CONS> <3> <NAT_NIL>)))",
    );
    let rev_src = (
        "<REV>",
        r#"(
    fun
    
    0
    
    (
        (<POLYMORPHIC_LIST> <NAT>) // reversee
        (<POLYMORPHIC_LIST> <NAT>) // out
    )
    
    (<POLYMORPHIC_LIST> <NAT>)
    
    (
        match 2 (<POLYMORPHIC_LIST> <NAT>)

        (
            (0 1)

            (2 
                // DB index stack
                // 0 => reversee.cdr
                // 1 => reversee.car
                // 2 => rev
                // 3 => out
                // 4 => reversee

                (2 0 (<NAT_CONS> 1 3))
            )
        )
    )
)"#,
    );
    let src_defs = [
        nat_def,
        zero_def,
        succ_def,
        one_def,
        two_def,
        three_def,
        list_0_def,
        polymorphic_list_def,
        polymorphic_nil_def,
        polymorphic_cons_def,
        nat_nil_def,
        cons_def,
        normalized_nat_list_def,
        normalized_nat_nil_def,
        normalized_nat_cons_def,
        one_two_three_src,
        rev_src,
    ];
    let rev_one_two_three_src = substitute_with_compounding(src_defs, r#"(<REV> <123> <NAT_NIL>)"#);

    use crate::{syntax_tree::rch_cst_to_ast::*, typecheck::*};

    let cst = parse_rch_cst_or_panic(&rev_one_two_three_src);
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
                "\n*****EXPR:*****\n{}\n\n*****EXPECTED TYPE:*****\n{}\n\n*****ACTUAL TYPE:*****\n{}\n\n****EXPR_SPAN:****\n{:?}\n\n****SRC:****:\n{}\n\n",
                PrettyPrinted(&RchCstToAstConverter::default().convert(expr.clone())),
                PrettyPrinted(expected_type.raw()),
                PrettyPrinted(actual_type.raw()),
                expr.span(),
                &rev_one_two_three_src,
            );
        }

        _ => panic!("expected TypeMismatch error, got {:?}", err),
    }
}
