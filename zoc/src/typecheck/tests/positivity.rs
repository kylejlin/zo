use super::*;

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

// TODO: Delete
// #[test]
// fn decreasing_equal() {
//     let nat_def = (
//         "<NAT>",
//         r#"
// (ind Set0 "Nat" () (
//     (() ())
//     ((0) ())
// ))"#,
//     );
//     let src_defs = [nat_def];

//     let unsubstituted_src = r#"
// (fun 0 (<NAT>) <NAT>
//     (match 2 1 <NAT> (
//         (0 1)
//         (1 (1 2))
//     ))
// )"#;

//     let src = substitute_with_compounding(src_defs, unsubstituted_src);
//     let err = get_type_error_under_empty_tcon_or_panic(&src);
//     let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
//     insta::assert_display_snapshot!(pretty_printed_err);
// }

// TODO: Add more tests.
