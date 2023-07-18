use crate::{pretty_print::PrettyPrint, test_utils::*};

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

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
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

    (<POLYMORPHIC_LIST> 0)

    (vcon <LIST_0> 0)
)"#,
    );
    let polymorphic_cons_def = (
        "<POLYMORPHIC_CONS>",
        r#"(
    fun

    nonrec

    (Type0)

    (for (0 (<POLYMORPHIC_LIST> 1)) (<POLYMORPHIC_LIST> 2))

    (vcon <LIST_0> 1)
)"#,
    );
    let nat_nil_def = ("<NAT_NIL>", "(<POLYMORPHIC_NIL> <NAT>)");
    let nat_cons_def = ("<NAT_CONS>", "(<POLYMORPHIC_CONS> <NAT>)");
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
        nat_cons_def,
        one_two_three_src,
        rev_src,
    ];
    let rev_one_two_three_src = substitute_with_compounding(src_defs, r#"(<REV> <123> <NAT_NIL>)"#);

    let type_ = get_type_under_empty_tcon_and_scon_or_panic(&rev_one_two_three_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

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

    (<POLYMORPHIC_LIST> 0)

    (vcon <LIST_0> 0)
)"#,
    );
    let polymorphic_cons_def = (
        "<POLYMORPHIC_CONS>",
        r#"(
    fun

    nonrec

    (Type0)

    (for (0 (<POLYMORPHIC_LIST> 1)) (<POLYMORPHIC_LIST> 2))

    (vcon <LIST_0> 1)
)"#,
    );
    let nat_nil_def = ("<NAT_NIL>", "(<POLYMORPHIC_NIL> <NAT>)");
    let nat_cons_def = ("<NAT_CONS>", "(<POLYMORPHIC_CONS> <NAT>)");
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
        nat_cons_def,
        one_two_three_src,
        rev_src,
    ];
    let polymorphic_rev_one_two_three_src =
        substitute_with_compounding(src_defs, r#"(<POLYMORPHIC_REV> <NAT> <123> <NAT_NIL>)"#);

    let type_ = get_type_under_empty_tcon_and_scon_or_panic(&polymorphic_rev_one_two_three_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn eq_zero_one() {
    let nat_def = (
        "<NAT>",
        r#"
(ind Type0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let zero_def = ("<ZERO>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let one_def = ("<1>", "(<SUCC> <ZERO>)");
    let eq_zero_def = (
        "<EQ_ZERO>",
        r#"
(ind Type0 "Eq0" (<NAT>) (
    (() (<ZERO>))
))"#,
    );
    let false_def = ("<FALSE>", r#"(ind Type0 "False" () ())"#);
    let eq_zero_one_implies_false_unsubstituted_src = r#"
(fun nonrec ((<EQ_ZERO> <1>)) <FALSE>
    (match 1 <FALSE> (
        contra
    ))
)"#;
    let src_defs = [nat_def, zero_def, succ_def, one_def, eq_zero_def, false_def];
    let eq_zero_one_implies_false_src =
        substitute_with_compounding(src_defs, eq_zero_one_implies_false_unsubstituted_src);

    let type_ = get_type_under_empty_tcon_and_scon_or_panic(&eq_zero_one_implies_false_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn eq_one_zero() {
    let nat_def = (
        "<NAT>",
        r#"
(ind Type0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let zero_def = ("<ZERO>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let one_def = ("<1>", "(<SUCC> <ZERO>)");
    let eq_one_def = (
        "<EQ_ONE>",
        r#"
(ind Type0 "Eq1" (<NAT>) (
    (() (<1>))
))"#,
    );
    let false_def = ("<FALSE>", r#"(ind Type0 "False" () ())"#);
    let eq_one_zero_implies_false_unsubstituted_src = r#"
(fun nonrec ((<EQ_ONE> <ZERO>)) <FALSE>
    (match 1 <FALSE> (
        contra
    ))
)"#;
    let src_defs = [nat_def, zero_def, succ_def, one_def, eq_one_def, false_def];
    let eq_one_zero_implies_false_src =
        substitute_with_compounding(src_defs, eq_one_zero_implies_false_unsubstituted_src);

    let type_ = get_type_under_empty_tcon_and_scon_or_panic(&eq_one_zero_implies_false_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn substitution_does_not_diverge_even_when_second_vcon_index_arg_is_subexpr_of_matchee_type_index_arg(
) {
    let direction_def = (
        "<DIRECTION>",
        r#"
(ind Type0 "Direction" () (
    (() ()) // North
    (() ()) // East
    (() ()) // South
    (() ()) // West
    ((0 1) ()) // Mix
))"#,
    );
    let north_def = ("<NORTH>", "(vcon <DIRECTION> 0)");
    let east_def = ("<EAST>", "(vcon <DIRECTION> 1)");
    let south_def = ("<SOUTH>", "(vcon <DIRECTION> 2)");
    let west_def = ("<WEST>", "(vcon <DIRECTION> 3)");
    let mix_def = ("<MIX>", "(vcon <DIRECTION> 4)");
    let northwest_def = ("<NORTHWEST>", "(<MIX> <NORTH> <WEST>)");
    let eq_north_def = (
        "<EQ_NORTH>",
        r#"
(ind Type0 "EqNorth" (<DIRECTION>) (
    (() (<NORTH>))
))"#,
    );
    let eq_south_def = (
        "<EQ_SOUTH>",
        r#"
(ind Type0 "EqSouth" (<DIRECTION>) (
    (() (<SOUTH>))
))"#,
    );
    let false_def = ("<FALSE>", r#"(ind Type0 "False" () ())"#);
    let implies_false_src_unsubstituted = r#"
    (fun nonrec ((<EQ_NORTH> <SOUTH>) (<EQ_SOUTH> <NORTHWEST>)) <FALSE>
        (match 2 <FALSE> (
            (
                0
                (match 1 <FALSE> (
                    contra
                ))
            )
        ))
    )"#;
    let src_defs = [
        direction_def,
        north_def,
        east_def,
        south_def,
        west_def,
        mix_def,
        northwest_def,
        eq_north_def,
        eq_south_def,
        false_def,
    ];
    let eq_zero_one_implies_false_src =
        substitute_with_compounding(src_defs, implies_false_src_unsubstituted);

    let type_ = get_type_under_empty_tcon_and_scon_or_panic(&eq_zero_one_implies_false_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn eq_commutative() {
    let bool_def = (
        "<BOOL>",
        r#"
(ind Type0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_def = ("<TRUE>", "(vcon <BOOL> 0)");
    let false_def = ("<FALSE>", "(vcon <BOOL> 1)");
    let eq_bool_def = (
        "<EQ>",
        r#"
(ind Type0 "Eq" (<BOOL> <BOOL>) (
    ((<BOOL>) (0 0))
))"#,
    );
    let unsubstituted_src_1 = r#"
(fun nonrec (<BOOL> <BOOL> (<EQ> 1 0)) (<EQ> 1 2)
    (match 1 (<EQ> 2 3) (
        (
            // Arity
            1
            // Return val
            (
                (vcon <EQ> 0)
                4 // First function param
            )
        )
    ))
)"#;
    let unsubstituted_src_2 = r#"
(fun nonrec (<BOOL> <BOOL> (<EQ> 1 0)) (<EQ> 1 2)
    (match 1 (<EQ> 2 3) (
        (
            // Arity
            1
            // Return val
            (
                (vcon <EQ> 0)
                3 // Second function param
            )
        )
    ))
)"#;
    let unsubstituted_src_3 = r#"
(fun nonrec (<BOOL> <BOOL> (<EQ> 1 0)) (<EQ> 1 2)
    (match 1 (<EQ> 2 3) (
        (
            // Arity
            1
            // Return val
            (
                (vcon <EQ> 0)
                0 // Match case param
            )
        )
    ))
)"#;
    let src_defs = [bool_def, true_def, false_def, eq_bool_def];
    let src_1 = substitute_with_compounding(src_defs, unsubstituted_src_1);
    let src_2 = substitute_with_compounding(src_defs, unsubstituted_src_2);
    let src_3 = substitute_with_compounding(src_defs, unsubstituted_src_3);

    let type_1 = get_type_under_empty_tcon_and_scon_or_panic(&src_1);
    let type_2 = get_type_under_empty_tcon_and_scon_or_panic(&src_2);
    let type_3 = get_type_under_empty_tcon_and_scon_or_panic(&src_3);

    assert_eq!(PrettyPrint(type_1.raw()), PrettyPrint(type_2.raw()));
    assert_eq!(PrettyPrint(type_1.raw()), PrettyPrint(type_3.raw()));

    insta::assert_display_snapshot!(PrettyPrint(type_1.raw()));
}

#[test]
fn eq_transitive() {
    let bool_def = (
        "<BOOL>",
        r#"
(ind Type0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_def = ("<TRUE>", "(vcon <BOOL> 0)");
    let false_def = ("<FALSE>", "(vcon <BOOL> 1)");
    let eq_bool_def = (
        "<EQ>",
        r#"
(ind Type0 "Eq" (<BOOL> <BOOL>) (
    ((<BOOL>) (0 0))
))"#,
    );
    let unsubstituted_src_1 = r#"
(fun nonrec (<BOOL> <BOOL> <BOOL> (<EQ> 2 1) (<EQ> 2 1)) (<EQ> 4 2)
    (match 2 (<EQ> 5 3) (
        (
            // Arity
            1

            // Return val
            (match 2 (<EQ> 6 4) (
                (
                    // Arity
                    1

                    // Return val
                    (
                        (vcon <EQ> 0)
                        7 // First function param
                    )
                )
            ))
        )
    ))
)"#;
    let unsubstituted_src_2 = r#"
(fun nonrec (<BOOL> <BOOL> <BOOL> (<EQ> 2 1) (<EQ> 2 1)) (<EQ> 4 2)
    (match 2 (<EQ> 5 3) (
        (
            // Arity
            1

            // Return val
            (match 2 (<EQ> 6 4) (
                (
                    // Arity
                    1

                    // Return val
                    (
                        (vcon <EQ> 0)
                        6 // Second function param
                    )
                )
            ))
        )
    ))
)"#;
    let unsubstituted_src_3 = r#"
(fun nonrec (<BOOL> <BOOL> <BOOL> (<EQ> 2 1) (<EQ> 2 1)) (<EQ> 4 2)
    (match 2 (<EQ> 5 3) (
        (
            // Arity
            1

            // Return val
            (match 2 (<EQ> 6 4) (
                (
                    // Arity
                    1

                    // Return val
                    (
                        (vcon <EQ> 0)
                        5 // Third function param
                    )
                )
            ))
        )
    ))
)"#;
    let unsubstituted_src_4 = r#"
(fun nonrec (<BOOL> <BOOL> <BOOL> (<EQ> 2 1) (<EQ> 2 1)) (<EQ> 4 2)
    (match 2 (<EQ> 5 3) (
        (
            // Arity
            1

            // Return val
            (match 2 (<EQ> 6 4) (
                (
                    // Arity
                    1

                    // Return val
                    (
                        (vcon <EQ> 0)
                        1 // Outer match case param
                    )
                )
            ))
        )
    ))
)"#;
    let unsubstituted_src_5 = r#"
(fun nonrec (<BOOL> <BOOL> <BOOL> (<EQ> 2 1) (<EQ> 2 1)) (<EQ> 4 2)
    (match 2 (<EQ> 5 3) (
        (
            // Arity
            1

            // Return val
            (match 2 (<EQ> 6 4) (
                (
                    // Arity
                    1

                    // Return val
                    (
                        (vcon <EQ> 0)
                        0 // Inner match case param
                    )
                )
            ))
        )
    ))
)"#;
    let src_defs = [bool_def, true_def, false_def, eq_bool_def];
    let src_1 = substitute_with_compounding(src_defs, unsubstituted_src_1);
    let src_2 = substitute_with_compounding(src_defs, unsubstituted_src_2);
    let src_3 = substitute_with_compounding(src_defs, unsubstituted_src_3);
    let src_4 = substitute_with_compounding(src_defs, unsubstituted_src_4);
    let src_5 = substitute_with_compounding(src_defs, unsubstituted_src_5);

    let type_1 = get_type_under_empty_tcon_and_scon_or_panic(&src_1);
    let type_2 = get_type_under_empty_tcon_and_scon_or_panic(&src_2);
    let type_3 = get_type_under_empty_tcon_and_scon_or_panic(&src_3);
    let type_4 = get_type_under_empty_tcon_and_scon_or_panic(&src_4);
    let type_5 = get_type_under_empty_tcon_and_scon_or_panic(&src_5);

    assert_eq!(PrettyPrint(type_1.raw()), PrettyPrint(type_2.raw()));
    assert_eq!(PrettyPrint(type_1.raw()), PrettyPrint(type_3.raw()));
    assert_eq!(PrettyPrint(type_1.raw()), PrettyPrint(type_4.raw()));
    assert_eq!(PrettyPrint(type_1.raw()), PrettyPrint(type_5.raw()));

    insta::assert_display_snapshot!(PrettyPrint(type_1.raw()));
}

#[ignore]
#[test]
fn index_arg_types_are_compared_against_substituted_written_index_types() {
    let src = r#"
    (ind Type1 "Precise" (Type0 0) (
        ((Type0 0) (1 0))
    ))
    "#;
    let type_ = get_type_under_empty_tcon_and_scon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}
