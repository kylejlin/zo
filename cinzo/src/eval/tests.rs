use super::*;

use pretty_assertions::assert_eq;

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
    let five_src = substitute_with_compounding(
        [nat_def, zero_def, succ_def],
        "(<SUCC> (<SUCC> (<SUCC> (<SUCC> (<SUCC> <ZERO>)))))",
    );

    let actual = {
        let tokens = crate::lexer::lex(&add_two_three_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        let ast: Expr = cst.into();
        Evaluator::default().eval(ast).unwrap().into_raw()
    };

    let expected = {
        let tokens = crate::lexer::lex(&five_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        Expr::from(cst)
    };

    assert_eq!(expected.digest(), actual.digest());
}

#[test]
fn nullary_match_case() {
    let dummy_ind_def = (
        "<DUMMY_IND>",
        r#"(ind Type0 "Dummy" () (
(() ())
((0) ())
((0 1) ())
))"#,
    );
    let match_src = substitute_with_compounding(
        [dummy_ind_def],
        r#"
(
    match (vcon <DUMMY_IND> 0) <DUMMY_IND> (
        (0 12)
        (1 14)
        (2 (16 1 0))
    )
)"#,
    );
    let expected_src = r#"12"#;

    let actual = {
        let tokens = crate::lexer::lex(&match_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        let ast: Expr = cst.into();
        Evaluator::default().eval(ast).unwrap().into_raw()
    };

    let expected = {
        let tokens = crate::lexer::lex(&expected_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        Expr::from(cst)
    };

    assert_eq!(expected.digest(), actual.digest());
}

#[test]
fn match_case_param_substitution() {
    let dummy_ind_def = (
        "<DUMMY_IND>",
        r#"(ind Type0 "Dummy" () (
(() ())
((0) ())
((0 1) ())
))"#,
    );
    let match_src = substitute_with_compounding(
        [dummy_ind_def],
        r#"
(
    match ((vcon <DUMMY_IND> 2) 10 11) <DUMMY_IND> (
        (0 12)
        (1 14)
        (2 (16 1 0))
    )
)"#,
    );
    let expected_src = r#"(14 10 11)"#;

    let actual = {
        let tokens = crate::lexer::lex(&match_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        let ast: Expr = cst.into();
        Evaluator::default().eval(ast).unwrap().into_raw()
    };

    let expected = {
        let tokens = crate::lexer::lex(&expected_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        Expr::from(cst)
    };

    assert_eq!(expected.digest(), actual.digest());
}

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
    let three_two_one_src =
            substitute_with_compounding(src_defs, "(<NORMALIZED_NAT_CONS> <3> (<NORMALIZED_NAT_CONS> <2> (<NORMALIZED_NAT_CONS> <1> <NORMALIZED_NAT_NIL>)))");

    let actual = {
        let tokens = crate::lexer::lex(&rev_one_two_three_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        let ast: Expr = cst.into();
        Evaluator::default().eval(ast).unwrap().into_raw()
    };

    let expected = {
        let tokens = crate::lexer::lex(&three_two_one_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        Expr::from(cst)
    };

    assert_eq!(expected.digest(), actual.digest());
}

#[ignore]
#[test]
fn polymorphic_rev_1_2_3() {
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
        "<POLYMORPHIC_REV>",
        r#"(
    fun
    
    1
    
    (
        Type0 // T
        (<POLYMORPHIC_LIST> 0) // reversee
        (<POLYMORPHIC_LIST> 1) // out
    )
    
    (<POLYMORPHIC_LIST> 2)
    
    (
        match 2 (<POLYMORPHIC_LIST> 3)

        (
            (0 1)

            (2 
                // DB index stack
                // 0 => reversee.cdr
                // 1 => reversee.car
                // 2 => rev
                // 3 => out
                // 4 => reversee
                // 5 => T

                (2 5 0 ((<POLYMORPHIC_CONS> 5) 1 3))
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
    let rev_one_two_three_src =
        substitute_with_compounding(src_defs, r#"(<POLYMORPHIC_REV> <NAT> <123> <NAT_NIL>)"#);
    let three_two_one_src =
            substitute_with_compounding(src_defs, "(<NORMALIZED_NAT_CONS> <3> (<NORMALIZED_NAT_CONS> <2> (<NORMALIZED_NAT_CONS> <1> <NORMALIZED_NAT_NIL>)))");

    let actual = {
        let tokens = crate::lexer::lex(&rev_one_two_three_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        let ast: Expr = cst.into();
        Evaluator::default().eval(ast).unwrap().into_raw()
    };

    let expected = {
        let tokens = crate::lexer::lex(&three_two_one_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        Expr::from(cst)
    };

    // TODO: Delete
    // std::fs::write("actual.txt", format!("{actual:#?}")).expect("failed to write file");
    // std::fs::write("expected.txt", format!("{expected:#?}")).expect("failed to write file");

    assert_eq!(expected.digest(), actual.digest());
}

#[test]
fn recursive_fun_app_stops_unfolding_when_decreasing_arg_not_vconlike() {
    let nat_def = (
        "<NAT>",
        r#"(ind Type0 "Nat" () (
(() ())
((0) ())
))"#,
    );
    let zero_def = ("<ZERO>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let recursive_identity_def = (
        "<RECURSIVE_IDENTITY>",
        r#"
(
    fun

    0

    (<NAT>)

    <NAT>

    (
        match 1 <NAT> (
            (0 <ZERO>)
            (1 (<SUCC> (1 0)))
        )
    )
)"#,
    );
    let defs = [nat_def, zero_def, succ_def, recursive_identity_def];
    let ident_succ_deb_123_src =
        substitute_with_compounding(defs, r#"(<RECURSIVE_IDENTITY> (<SUCC> 123))"#);
    let succ_ident_deb_123_src =
        substitute_with_compounding(defs, "(<SUCC> (<RECURSIVE_IDENTITY> 123))");

    let actual = {
        let tokens = crate::lexer::lex(&ident_succ_deb_123_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        let ast: Expr = cst.into();
        Evaluator::default().eval(ast).unwrap().into_raw()
    };

    let expected = {
        let tokens = crate::lexer::lex(&succ_ident_deb_123_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        Expr::from(cst)
    };

    assert_eq!(expected.digest(), actual.digest());
}

#[ignore]
#[test]
fn substitution_upshifts_new_expr_debs() {
    let dummy_ind_def = (
        "<DUMMY_IND>",
        r#"
(
    ind

    Type0

    "DummyInd"

    ()

    (
        ((200) ())
        ((220 240) ())
    )
)"#,
    );
    let match_src = substitute_with_compounding(
        [dummy_ind_def],
        r#"
(
    match ((vcon <DUMMY_IND> 1) 5 ((vcon <DUMMY_IND> 0) 100)) 120 (
        (1 140)

        (
            2

            (
                match 0 160 (
                    (1 2)

                    (2 180)
                )
            )
        )
    )
)"#,
    );
    let deb_5_src = "5";

    let actual = {
        let tokens = crate::lexer::lex(&match_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        let ast: Expr = cst.into();
        Evaluator::default().eval(ast).unwrap().into_raw()
    };

    let expected = {
        let tokens = crate::lexer::lex(&deb_5_src).unwrap();
        let cst = crate::parser::parse(tokens).unwrap();
        Expr::from(cst)
    };

    assert_eq!(expected.digest(), actual.digest());
}

fn substitute_with_compounding<'a>(
    iter: impl IntoIterator<Item = (&'a str, &'a str)>,
    last: &'a str,
) -> String {
    let mut replacements = vec![];
    for (from, unreplaced_to) in iter {
        let to = substitute_without_compounding(&replacements, unreplaced_to);
        replacements.push((from, to));
    }
    substitute_without_compounding(&replacements, last)
}

fn substitute_without_compounding(replacements: &[(&str, String)], original: &str) -> String {
    let mut result = original.to_string();
    for (from, to) in replacements {
        result = result.replace(from, to);
    }
    result
}
