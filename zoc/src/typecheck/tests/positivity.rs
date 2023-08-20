use super::*;

// The word "legal" and "illegal" look too similar.
// When I'm skimming the (usually long) test names, I want to
// be able to distinguish legal tests
// from illegal tests in a split-second.
// The similarity between the two words makes this hard.
//
// So, I use "ok" instead of "legal",
// and "ng" (short for "no good") instead of "illegal".

#[test]
fn rec_ind_in_index_arg_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(fun nonrec ((for (Set1) Set0)) (for (Set0) Set1)
    (ind Set1 "Foo" (Set0) (
        (
            // vcon param types
            ()

            // index args
            (
                (2 (0 <FALSE>))
            )
        )
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn nonrecursive_param_types_are_ok() {
    let nat_def = (
        "<NAT>",
        r#"
(ind Set0 "Nat" () (
    (() ())
    ((0) ())
))"#,
    );
    let src_defs = [nat_def];

    let unsubstituted_src = r#"
(ind Set0 "NatPair" () (
    ((<NAT> <NAT>) ())
))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn recursive_ind_param_types_are_ng() {
    let src = r#"
(ind Set0 "Tree" () (
    // `leaf`
    (
        // vcon param types
        ()

        // index args
        ()
    )

    // `pair`
    (
        // vcon param types
        (0 1)

        // index args
        ()
    )
))"#;
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn recursive_ind_app_with_nonrecursive_arg_as_param_types_are_ok() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(ind Set1 "Foo" (Set0) (
    (
        // vcon param types
        (
            (0 <FALSE>)
            (1 <FALSE>)
        )

        // index args
        (
            <FALSE>
        )
    )
))"#;
    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn recursive_ind_app_with_recursive_ind_in_arg_as_first_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(fun nonrec ((for (Set1) Set0)) (for (Set0) Set1)
    (ind Set1 "Foo" (Set0) (
        (
            // vcon param types
            (
                (
                    0
                    (2 (0 <FALSE>))
                )
            )

            // index args
            (
                <FALSE>
            )
        )
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn recursive_ind_app_with_recursive_ind_in_arg_as_second_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(fun nonrec ((for (Set1) Set0)) (for (Set0) Set1)
    (ind Set1 "Foo" (Set0) (
        (
            // vcon param types
            (
                <FALSE>
                
                (
                    1
                    (3 (1 <FALSE>))
                )
            )

            // index args
            (
                <FALSE>
            )
        )
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn strictly_positive_fors_in_param_types_are_ok() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(ind Set0 "Negative" () (
    (
        // vcon param types
        (
            (for (<FALSE>) 1)

            (for (<FALSE> <FALSE>) (for (<FALSE>) 4))
        )
        
        // index args
        ()
    )
))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn negative_recursive_ind_in_first_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(ind Set0 "Negative" () (
    (((for (0) <FALSE>)) ())
))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn nonstrictly_positive_recursive_ind_in_first_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(ind Set0 "NonstrictlyPositive" () (
    (((for ((for (0) <FALSE>)) <FALSE>)) ())
))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn for_with_return_type_with_nonstrictly_positive_recursive_ind_in_first_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(ind Set0 "NonstrictlyPositive" () (
    (
        // vcon param types
        (
            (
                for

                (<FALSE>)

                (for ((for (1) <FALSE>)) <FALSE>)
            )
        )
        
        // index types
        ()
    )
))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn negative_recursive_ind_in_second_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(ind Set0 "Negative" () (
    ((<FALSE> (for (1) <FALSE>)) ())
))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn nonstrictly_positive_recursive_ind_in_second_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(ind Set0 "NonstrictlyPositive" () (
    ((<FALSE> (for ((for (1) <FALSE>)) <FALSE>)) ())
))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn for_with_return_type_with_nonstrictly_positive_recursive_ind_in_second_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(ind Set0 "NonstrictlyPositive" () (
    (
        // vcon param types
        (
            <FALSE>

            (
                for

                (<FALSE>)

                (for ((for (2) <FALSE>)) <FALSE>)
            )
        )
        
        // index types
        ()
    )
))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn positivity_condition_satisfying_ind_expr_as_param_types_are_ok() {
    let src = r#"
(ind Set0 "Tree" () (
    // `leaf`
    (() ())

    // `internal`
    (
        // vcon param types
        (
            (ind Set0 "List" () (
                // nil
                (() ())
            
                // cons
                ((1 1) ())
            ))

            (ind Set0 "List" () (
                // nil
                (() ())
            
                // cons
                ((2 1) ())
            ))
        )
        
        // index args
        ()
    )
))"#;
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn app_with_positivity_condition_satisfying_ind_callee_and_nonrec_args_as_param_types_are_ok() {
    let true_def = (
        "<TRUE>",
        r#"
(ind Set0 "True" () (
    (() ())
))"#,
    );
    let truec_def = ("<TRUEC>", r#"(vcon <TRUE> 0)"#);
    let src_defs = [true_def, truec_def];

    let unsubstituted_src = r#"
    (ind Set0 "Tree" () (
        // `leaf`
        (() ())
    
        // `internal`
        (
            // vcon param types
            (
                // first vcon param type 
                (
                    (ind Set0 "List" (<TRUE>) (
                        // nil
                        (() (<TRUEC>))
                    
                        // cons
                        ((1 (1 <TRUEC>)) (<TRUEC>))
                    ))

                    <TRUEC>
                )
    
                // second vcon param type 
                (
                    (ind Set0 "List" (<TRUE>) (
                        // nil
                        (() (<TRUEC>))
                    
                        // cons
                        ((2 (1 <TRUEC>)) (<TRUEC>))
                    ))

                    <TRUEC>
                )
            )
            
            // index args
            ()
        )
    ))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn ind_app_with_recursive_ind_in_arg_as_first_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(fun nonrec ((for (Set1) Set0)) (for (Set0) Set1)
    (ind Set1 "Foo" (Set0) (
        (
            // vcon param types
            (
                (
                    (ind Set1 "Bar" (Set0) (
                        (() (<FALSE>))
                    ))

                    (2 (0 <FALSE>))
                )
            )

            // index args
            (<FALSE>)
        )
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn ind_app_with_recursive_ind_in_arg_as_second_param_type_is_ng() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(fun nonrec ((for (Set1) Set0)) (for (Set0) Set1)
    (ind Set1 "Foo" (Set0) (
        (
            // vcon param types
            (
                <FALSE>

                (
                    (ind Set1 "Bar" (Set0) (
                        (() (<FALSE>))
                    ))

                    (3 (1 <FALSE>))
                )
            )

            // index args
            (<FALSE>)
        )
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn tree_inline() {
    let src = r#"
(ind Set0 "Tree" () (
    // `leaf`
    (() ())

    // `internal`
    ((
        (ind Set0 "List" () (
            // nil
            (() ())
        
            // cons
            ((1 1) ())
        ))
    ) ())
))"#;
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn list_tree_inline() {
    let tree_def = (
        "<TREE>",
        r#"
(ind Set0 "Tree" () (
    // `leaf`
    (() ())

    // `internal`
    ((
        (ind Set0 "List" () (
            // nil
            (() ())
        
            // cons
            ((1 1) ())
        ))
    ) ())
))"#,
    );
    let src_defs = [tree_def];

    let unsubstituted_src = r#"
    (ind Set0 "List" () (
        // DB index stack is
        // 0 =>  List(T)
        // 1 => List 
        // 2 => T
    
        // nil
        (() ())
    
        // cons
        ((
            <TREE>
    
            // DB index stack is
            // 0 => car
            // 1 => List(T)
            // 2 => List
            // 3 => T
            1
        ) ())
    ))"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn tree_noninline() {
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
    let tree_def = (
        "<TREE>",
        r#"
(ind Set0 "Tree" () (
    // `leaf`
    (() ())

    // `internal`
    (((<POLYMORPHIC_LIST> 0) 1) ())
))"#,
    );
    let src_defs = [
        list_0_def,
        polymorphic_list_def,
        polymorphic_nil_def,
        polymorphic_cons_def,
        tree_def,
    ];

    let unsubstituted_src = r#"<TREE>"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn list_tree_noninline() {
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
    let tree_def = (
        "<TREE>",
        r#"
(ind Set0 "Tree" () (
    // `leaf`
    (() ())

    // `internal`
    (((<POLYMORPHIC_LIST> 0) 1) ())
))"#,
    );
    let src_defs = [
        list_0_def,
        polymorphic_list_def,
        polymorphic_nil_def,
        polymorphic_cons_def,
        tree_def,
    ];

    let unsubstituted_src = r#"(<POLYMORPHIC_LIST> <TREE>)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

// TODO: Add more tests.
