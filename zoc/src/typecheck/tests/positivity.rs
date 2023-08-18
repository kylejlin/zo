use super::*;

#[test]
fn rec_ind_in_index_arg_is_illegal() {
    let false_def = (
        "<FALSE>",
        r#"
(ind Set0 "False" () ())"#,
    );
    let src_defs = [false_def];

    let unsubstituted_src = r#"
(fun nonrec ((for (Set1) Set0)) Set0
    (ind Set1 "Foo" (Set0) (
        (
            // vcon params
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
fn strictly_positive_params_are_legal() {
    let src = r#"
(ind Set0 "Tree" () (
    // `leaf`
    (
        // param types
        ()

        // index args
        ()
    )

    // `pair`
    (
        // param types
        (0 1)

        // index args
        ()
    )
))"#;
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn negative_appearance_in_first_param_type_is_illegal() {
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
fn nonstrictly_positive_in_first_param_type_is_illegal() {
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
fn negative_appearance_in_second_param_type_is_illegal() {
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
fn nonstrictly_positive_in_second_param_type_is_illegal() {
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
fn tree() {
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

    let unsubstituted_src = r#"<TREE>"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let type_ = get_type_under_empty_tcon_or_panic(&src);
    insta::assert_display_snapshot!(PrettyPrint(type_.raw()));
}

#[test]
fn inline_list_tree() {
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
fn noninline_tree() {
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
fn noninline_list_tree() {
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
