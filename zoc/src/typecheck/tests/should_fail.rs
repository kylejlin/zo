use super::*;

#[test]
fn wrong_match_return_type_arity() {
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
    
    (match
        // Matchee
        1

        // Correct return type arity: 3
        // What we write: 2
        2

        // Return type
        (<EQ> 1 2) 

        // Cases
        (
            (
                // Arity
                1
                // Return val
                (
                    (vcon <EQ> 0)
                    0
                )
            )
        )
    )
)"#;

    let src_defs = [bool_def, true_def, false_def, eq_bool_def];
    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);

    insta::assert_debug_snapshot!(err);
}

#[test]
fn app_has_zero_args() {
    let tcon_entry_src = r#"
(for
    ()
    
    (ind Set0 "Unit" () (
        (() ())
    ))
)"#;
    let tcon_entry = eval_or_panic(tcon_entry_src);
    let tcon_entries: Normalized<Vec<_>> = std::iter::once(tcon_entry).collect();
    let tcon = LazyTypeContext::Base(tcon_entries.to_derefed());

    let src = "(0)";
    let err = get_type_error_or_panic(&src, tcon);

    insta::assert_debug_snapshot!(err);
}

#[test]
fn fun_has_zero_args() {
    let bool_def = (
        "<BOOL>",
        r#"
(ind Set0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_def = ("<TRUE>", "(vcon <BOOL> 0)");
    let unsubstituted_src = r#"
(fun nonrec () <BOOL>
    <TRUE>
)"#;

    let src_defs = [bool_def, true_def];
    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_type_error_under_empty_tcon_or_panic(&src);

    insta::assert_debug_snapshot!(err);
}

// TODO: Add more tests.
