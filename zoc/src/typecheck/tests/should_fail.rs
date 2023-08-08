use super::*;

#[test]
fn wrong_match_return_type_arity() {
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

    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

// TODO: Add more tests.
