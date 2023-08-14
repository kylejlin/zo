use super::*;

#[test]
fn add_2_3() {
    let nat_def = (
        "<NAT>",
        r#"(ind Set0 "Nat" () (
(() ())
((0) ())
))"#,
    );
    let zero_def = ("<0>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let two_def = ("<2>", "(<SUCC> (<SUCC> <0>))");
    let three_def = ("<3>", "(<SUCC> <2>)");
    let add_tailcall_impl_def = (
        "<ADD_TAILCALL_IMPL>",
        "
(fun 0 (<NAT> <NAT>) <NAT>
    (match 2 1 <NAT> (
        (0 1)
        (1 (1 0 (<SUCC> 2)))
    ))
)",
    );
    let add_two_three_src = substitute_with_compounding(
        [
            nat_def,
            zero_def,
            succ_def,
            two_def,
            three_def,
            add_tailcall_impl_def,
        ],
        r#"(<ADD_TAILCALL_IMPL> <2> <3>)"#,
    );

    let type_ = get_type_under_empty_tcon_or_panic(&add_two_three_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn rev_1_2_3() {
    let nat_def = (
        "<NAT>",
        r#"
(ind Set0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let zero_def = ("<0>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let one_def = ("<1>", "(<SUCC> <0>)");
    let two_def = ("<2>", "(<SUCC> <1>)");
    let three_def = ("<3>", "(<SUCC> <2>)");
    let list_0_def = (
        "<LIST_0>",
        r#"
(ind Set0 "List" () (
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
))"#,
    );
    let polymorphic_list_def = (
        "<POLYMORPHIC_LIST>",
        r#"
(fun nonrec (Set0) Set0
    <LIST_0>
)"#,
    );
    let polymorphic_nil_def = (
        "<POLYMORPHIC_NIL>",
        r#"
(fun nonrec (Set0) (<POLYMORPHIC_LIST> 0)
    (vcon <LIST_0> 0)
)"#,
    );
    let polymorphic_cons_def = (
        "<POLYMORPHIC_CONS>",
        r#"
(fun nonrec (Set0) (for (0 (<POLYMORPHIC_LIST> 1)) (<POLYMORPHIC_LIST> 2))
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
        r#"
(
    fun
    
    0
    
    (
        (<POLYMORPHIC_LIST> <NAT>) // reversee
        (<POLYMORPHIC_LIST> <NAT>) // out
    )
    
    (<POLYMORPHIC_LIST> <NAT>)
    
    (
        match 2 1 (<POLYMORPHIC_LIST> <NAT>)

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

    let type_ = get_type_under_empty_tcon_or_panic(&rev_one_two_three_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn polymorphic_rev_1_2_3() {
    let nat_def = (
        "<NAT>",
        r#"
(ind Set0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let zero_def = ("<0>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let one_def = ("<1>", "(<SUCC> <0>)");
    let two_def = ("<2>", "(<SUCC> <1>)");
    let three_def = ("<3>", "(<SUCC> <2>)");
    let list_0_def = (
        "<LIST_0>",
        r#"
(ind Set0 "List" () (
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
))"#,
    );
    let polymorphic_list_def = (
        "<POLYMORPHIC_LIST>",
        r#"
(fun nonrec (Set0) Set0
    <LIST_0>
)"#,
    );
    let polymorphic_nil_def = (
        "<POLYMORPHIC_NIL>",
        r#"
(fun nonrec (Set0) (<POLYMORPHIC_LIST> 0)
    (vcon <LIST_0> 0)
)"#,
    );
    let polymorphic_cons_def = (
        "<POLYMORPHIC_CONS>",
        r#"
(fun nonrec (Set0) (for (0 (<POLYMORPHIC_LIST> 1)) (<POLYMORPHIC_LIST> 2))
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
        r#"
(
    fun
    
    1
    
    (
        Set0 // T
        (<POLYMORPHIC_LIST> 0) // reversee
        (<POLYMORPHIC_LIST> 1) // out
    )
    
    (<POLYMORPHIC_LIST> 2)
    
    (
        match 2 1 (<POLYMORPHIC_LIST> 4)

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

    let type_ = get_type_under_empty_tcon_or_panic(&polymorphic_rev_one_two_three_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn ex_falso() {
    let nat_def = (
        "<NAT>",
        r#"
(ind Set0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let false_def = ("<FALSE>", r#"(ind Prop0 "False" () ())"#);
    let src_defs = [nat_def, false_def];
    let unsubstituted_src = r#"
    (fun nonrec (<FALSE>) <NAT>
        (match 1 1 <NAT> ())
    )"#;
    let src = substitute_with_compounding(src_defs, unsubstituted_src);

    let type_ = get_type_under_empty_tcon_or_panic(&src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn eq_zero_one() {
    let nat_def = (
        "<NAT>",
        r#"
(ind Set0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let zero_def = ("<0>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let one_def = ("<1>", "(<SUCC> <0>)");
    let eq_zero_def = (
        "<EQ_0>",
        r#"
(ind Prop0 "Eq0" (<NAT>) (
    (() (<0>))
))"#,
    );
    let false_def = ("<FALSE>", r#"(ind Prop0 "False" () ())"#);
    let unit_def = ("<UNIT>", r#"(ind Prop0 "Unit" () ((() ())))"#);
    let unitc_def = ("<UNITC>", "(vcon <UNIT> 0)");
    let is_zero_predicate_def = (
        "<IS_ZERO_PREDICATE>",
        r#"
(fun nonrec (<NAT>) Prop0
    (match 1 1 Prop0 (
        (0 <UNIT>)
        (1 <FALSE>)
    ))
)"#,
    );
    let eq_zero_one_implies_false_unsubstituted_src = r#"
(fun nonrec ((<EQ_0> <1>)) <FALSE>
    (match 1 2 (<IS_ZERO_PREDICATE> 1) (
        (0 <UNITC>)
    ))
)"#;
    let src_defs = [
        nat_def,
        zero_def,
        succ_def,
        one_def,
        eq_zero_def,
        false_def,
        unit_def,
        unitc_def,
        is_zero_predicate_def,
    ];
    let eq_zero_one_implies_false_src =
        substitute_with_compounding(src_defs, eq_zero_one_implies_false_unsubstituted_src);

    let type_ = get_type_under_empty_tcon_or_panic(&eq_zero_one_implies_false_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn eq_one_zero() {
    let nat_def = (
        "<NAT>",
        r#"
(ind Set0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let zero_def = ("<0>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let one_def = ("<1>", "(<SUCC> <0>)");
    let eq_one_def = (
        "<EQ_1>",
        r#"
(ind Prop0 "Eq1" (<NAT>) (
    (() (<1>))
))"#,
    );
    let false_def = ("<FALSE>", r#"(ind Prop0 "False" () ())"#);
    let unit_def = ("<UNIT>", r#"(ind Prop0 "Unit" () ((() ())))"#);
    let unitc_def = ("<UNITC>", "(vcon <UNIT> 0)");
    let is_one_predicate_def = (
        "<IS_ONE_PREDICATE>",
        r#"
(fun nonrec (<NAT>) Prop0
    (match 1 1 Prop0 (
        (0 <FALSE>)
        (1 (match 0 1 Prop0 (
            (0 <UNIT>)
            (1 <FALSE>)
        )))
    ))
)"#,
    );
    let eq_one_zero_implies_false_unsubstituted_src = r#"
(fun nonrec ((<EQ_1> <0>)) <FALSE>
    (match 1 2 (<IS_ONE_PREDICATE> 1) (
        (0 <UNITC>)
    ))
)"#;
    let src_defs = [
        nat_def,
        zero_def,
        succ_def,
        one_def,
        eq_one_def,
        false_def,
        unit_def,
        unitc_def,
        is_one_predicate_def,
    ];
    let eq_one_zero_implies_false_src =
        substitute_with_compounding(src_defs, eq_one_zero_implies_false_unsubstituted_src);

    let type_ = get_type_under_empty_tcon_or_panic(&eq_one_zero_implies_false_src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn eq_commutative() {
    let bool_def = (
        "<BOOL>",
        r#"
(ind Set0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_def = ("<TRUE>", "(vcon <BOOL> 0)");
    let false_def = ("<FALSE>", "(vcon <BOOL> 1)");
    let eq_bool_def = (
        "<EQ>",
        r#"
(ind Prop0 "Eq" (<BOOL> <BOOL>) (
    ((<BOOL>) (0 0))
))"#,
    );
    let unsubstituted_src = r#"
(fun nonrec (<BOOL> <BOOL> (<EQ> 1 0)) (<EQ> 1 2)
    (match 1 3 (<EQ> 1 2) (
        (
            // Arity
            1
            // Return val
            (
                (vcon <EQ> 0)
                0
            )
        )
    ))
)"#;

    let src_defs = [bool_def, true_def, false_def, eq_bool_def];
    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);

    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn eq_transitive() {
    let bool_def = (
        "<BOOL>",
        r#"
(ind Set0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_def = ("<TRUE>", "(vcon <BOOL> 0)");
    let false_def = ("<FALSE>", "(vcon <BOOL> 1)");
    let eq_bool_def = (
        "<EQ>",
        r#"
(ind Prop0 "Eq" (<BOOL> <BOOL>) (
    ((<BOOL>) (0 0))
))"#,
    );
    let unsubstituted_src = r#"
(fun nonrec (<BOOL> <BOOL> <BOOL> (<EQ> 2 1) (<EQ> 2 1)) (<EQ> 4 2)
    (
        (match 2 3 (for ((<EQ> 1 6)) (<EQ> 3 7)) (
            (
                // Arity
                1
    
                // Return val
                (fun nonrec ((<EQ> 0 4)) (<EQ> 1 5) 1)
            )
        ))

        1
    )
)"#;
    let src_defs = [bool_def, true_def, false_def, eq_bool_def];
    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn vcon_index_arg_types_are_compared_against_ind_index_types_substituted_with_vcon_index_args() {
    let src = r#"
    (ind Set1 "Precise" (Set0 0) (
        ((Set0 0) (1 0))
    ))
    "#;
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn add_zero() {
    let nat_def = (
        "<NAT>",
        r#"(ind Set0 "Nat" () (
(() ())
((0) ())
))"#,
    );
    let zero_def = ("<0>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let add_def = (
        "<ADD>",
        "
(fun 0 (<NAT> <NAT>) <NAT>
    (match 2 1 <NAT> (
        (0 1)
        (1 (<SUCC> (1 0 2)))
    ))
)",
    );
    let eq_def = (
        "<EQ>",
        r#"
(fun nonrec (<NAT> <NAT>) Prop0
    (
        (ind Prop0 "Eq" (<NAT>) (
            (() (3))
        ))
        1
    )
)"#,
    );
    let refl_def = (
        "<REFL>",
        r#"
(fun nonrec (<NAT>) (<EQ> 0 0)
    (
        vcon
        (ind Prop0 "Eq" (<NAT>) (
            (() (2))
        ))
        0
    )
)"#,
    );
    let src_defs = [nat_def, zero_def, succ_def, add_def, eq_def, refl_def];

    let unsubstituted_src = r#"
(fun 0 (<NAT>) (<EQ> 0 (<ADD> 0 <0>))
    (match 1 1 (<EQ> 0 (<ADD> 0 <0>)) (
        // `zero` case
        (
            // Case arity
            0

            // Return val
            (<REFL> <0>)
        )

        // `(succ pred)` case
        (
            // Case arity
            1

            // Return val
            //   [goal: (EQ (SUCC 0) (ADD (SUCC 0) ZERO))]
            //   [goal: (EQ (SUCC 0) (SUCC (ADD 0 ZERO)))]
            //   [(1 0): (EQ 0 (ADD 0 ZERO))]
            (match (1 0) 2 (<EQ> (<SUCC> 2) (<SUCC> 1)) (
                // `refl` case (only case)
                (
                    // Case arity
                    0

                    // Return val
                    (<REFL> (<SUCC> 0))
                )
            ))
        )
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn add_succ() {
    let nat_def = (
        "<NAT>",
        r#"(ind Set0 "Nat" () (
(() ())
((0) ())
))"#,
    );
    let zero_def = ("<0>", "(vcon <NAT> 0)");
    let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
    let add_def = (
        "<ADD>",
        "
(fun 0 (<NAT> <NAT>) <NAT>
    (match 2 1 <NAT> (
        (0 1)
        (1 (<SUCC> (1 0 2)))
    ))
)",
    );
    let eq_def = (
        "<EQ>",
        r#"
(fun nonrec (<NAT> <NAT>) Prop0
    (
        (ind Prop0 "Eq" (<NAT>) (
            (() (3))
        ))
        1
    )
)"#,
    );
    let refl_def = (
        "<REFL>",
        r#"
(fun nonrec (<NAT>) (<EQ> 0 0)
    (
        vcon
        (ind Prop0 "Eq" (<NAT>) (
            (() (2))
        ))
        0
    )
)"#,
    );
    let src_defs = [nat_def, zero_def, succ_def, add_def, eq_def, refl_def];

    let unsubstituted_src = r#"
(fun 0 (<NAT> <NAT>) (<EQ> (<ADD> 1 (<SUCC> 0)) (<SUCC> (<ADD> 1 0)))
    (match 2 1 (<EQ> (<ADD> 0 (<SUCC> 2)) (<SUCC> (<ADD> 0 2))) (
        // `zero` case
        (
            // Case arity
            0

            // Return val
            (<REFL> (<SUCC> 1))
        )

        // `(succ pred)` case
        (
            // Case arity
            1

            // Return val
            //   [goal: (EQ (ADD (SUCC 0) (SUCC 2)) (SUCC (ADD (SUCC 0) 2)))]
            //   [goal: (EQ (SUCC (ADD 0 (SUCC 2))) (SUCC (SUCC (ADD 0 2))))]
            //   [(1 0 2): (EQ (ADD 0 (SUCC 2)) (SUCC (ADD 0 2)))]
            (match (1 0 2) 2 (<EQ> (<SUCC> (<ADD> 2 (<SUCC> 4))) (<SUCC> 1)) (
                // `refl` case (only case)
                (
                    // Case arity
                    0

                    // Return val
                    (<REFL> (<SUCC> (<ADD> 0 (<SUCC> 2))))
                )
            ))
        )
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn substituting_recursive_ind_deb_for_ind_stops_after_one_level() {
    let list_0_def = (
        "<LIST_0>",
        r#"
(ind Set0 "List" () (
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
))"#,
    );
    let polymorphic_list_def = (
        "<POLYMORPHIC_LIST>",
        r#"
(fun nonrec (Set0) Set0
    <LIST_0>
)"#,
    );
    let polymorphic_nil_def = (
        "<POLYMORPHIC_NIL>",
        r#"
(fun nonrec (Set0) (<POLYMORPHIC_LIST> 0)
    (vcon <LIST_0> 0)
)"#,
    );
    let polymorphic_cons_def = (
        "<POLYMORPHIC_CONS>",
        r#"
(fun nonrec (Set0) (for (0 (<POLYMORPHIC_LIST> 1)) (<POLYMORPHIC_LIST> 2))
    (vcon <LIST_0> 1)
)"#,
    );
    let mutual_def = (
        "<MUTUAL>",
        r#"
(ind Set0 "Mutual" () (
    (((<POLYMORPHIC_LIST> 0) 1) ())
))"#,
    );
    let src_defs = [
        list_0_def,
        polymorphic_list_def,
        polymorphic_nil_def,
        polymorphic_cons_def,
        mutual_def,
    ];

    let unsubstituted_src = r#"(vcon <MUTUAL> 0)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}
