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
