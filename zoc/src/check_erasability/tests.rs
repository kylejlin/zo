use crate::{pretty_print::*, test_utils::*};

#[test]
fn ng_2_variant_erasable_to_nonerasable() {
    let bool_prop_def = (
        "<BOOL_PROP>",
        r#"
(ind Prop0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let bool_set_def = (
        "<BOOL_SET>",
        r#"
(ind Set0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_set_def = ("<TRUE_SET>", r#"(vcon <BOOL_SET> 0)"#);
    let false_set_def = ("<FALSE_SET>", r#"(vcon <BOOL_SET> 1)"#);
    let src_defs = [bool_prop_def, bool_set_def, true_set_def, false_set_def];

    let unsubstituted_src = r#"
(fun nonrec (<BOOL_PROP>) <BOOL_SET>
    (match 1 1 <BOOL_SET> (
        (0 <TRUE_SET>)
        (0 <FALSE_SET>)
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_erasability_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn ng_1_variant_erasable_with_at_least_1_nonerasable_vcon_def_param_type_to_nonerasable() {
    let bool_prop_def = (
        "<BOOL_PROP>",
        r#"
(ind Prop0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let bool_set_def = (
        "<BOOL_SET>",
        r#"
(ind Set0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let foo_def = (
        "<FOO>",
        r#"
(ind Prop0 "Foo" () (
    ((<BOOL_PROP> <BOOL_PROP> <BOOL_SET>) ())
))"#,
    );
    let src_defs = [bool_prop_def, bool_set_def, foo_def];

    let unsubstituted_src = r#"
(fun nonrec (<FOO>) <BOOL_SET>
    (match 1 1 <BOOL_SET> (
        (3 0)
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    let err = get_erasability_error_under_empty_tcon_or_panic(&src);
    let pretty_printed_err = format!("{:#}", PrettyPrint(&err));
    insta::assert_display_snapshot!(pretty_printed_err);
}

#[test]
fn ok_1_variant_erasable_with_all_erasable_vcon_def_param_types_to_nonerasable() {
    let bool_prop_def = (
        "<BOOL_PROP>",
        r#"
(ind Prop0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let bool_set_def = (
        "<BOOL_SET>",
        r#"
(ind Set0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_set_def = ("<TRUE_SET>", r#"(vcon <BOOL_SET> 0)"#);
    let foo_def = (
        "<FOO>",
        r#"
(ind Prop0 "Foo" () (
    ((<BOOL_PROP> <BOOL_PROP> <BOOL_PROP>) ())
))"#,
    );
    let src_defs = [bool_prop_def, bool_set_def, true_set_def, foo_def];

    let unsubstituted_src = r#"
(fun nonrec (<FOO>) <BOOL_SET>
    (match 1 1 <BOOL_SET> (
        (3 <TRUE_SET>)
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    check_erasability_under_empty_tcon_or_panic(&src);
}

#[test]
fn ok_0_variant_erasable_to_nonerasable() {
    let empty_prop_def = (
        "<EMPTY_PROP>",
        r#"
(ind Prop0 "Empty" () ())"#,
    );
    let bool_set_def = (
        "<BOOL_SET>",
        r#"
(ind Set0 "Bool" () (
    (() ())
    (() ())
))"#,
    );

    let src_defs = [empty_prop_def, bool_set_def];

    let unsubstituted_src = r#"
(fun nonrec (<EMPTY_PROP>) <BOOL_SET>
    (match 1 1 <BOOL_SET> ())
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    check_erasability_under_empty_tcon_or_panic(&src);
}

#[test]
fn ok_erasable_with_at_least_1_nonerasable_vcon_def_param_type_to_erasable() {
    let bool_prop_def = (
        "<BOOL_PROP>",
        r#"
(ind Prop0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_prop_def = ("<TRUE_PROP>", "(vcon <BOOL_PROP> 0)");
    let bool_set_def = (
        "<BOOL_SET>",
        r#"
(ind Set0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let foo_def = (
        "<FOO>",
        r#"
(ind Prop0 "Foo" () (
    ((<BOOL_SET>) ())
))"#,
    );
    let src_defs = [bool_prop_def, true_prop_def, bool_set_def, foo_def];

    let unsubstituted_src = r#"
(fun nonrec (<FOO>) <BOOL_PROP>
    (match 1 1 <BOOL_PROP> (
        (1 <TRUE_PROP>)
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    check_erasability_under_empty_tcon_or_panic(&src);
}

#[test]
fn ok_2_variant_erasable_to_erasable() {
    let bool_prop_def = (
        "<BOOL_PROP>",
        r#"
(ind Prop0 "Bool" () (
    (() ())
    (() ())
))"#,
    );
    let true_prop_def = ("<TRUE_PROP>", "(vcon <BOOL_PROP> 0)");
    let src_defs = [bool_prop_def, true_prop_def];

    let unsubstituted_src = r#"
(fun nonrec (<BOOL_PROP>) <BOOL_PROP>
    (match 1 1 <BOOL_PROP> (
        (0 <TRUE_PROP>)
        (0 <TRUE_PROP>)
    ))
)"#;

    let src = substitute_with_compounding(src_defs, unsubstituted_src);
    check_erasability_under_empty_tcon_or_panic(&src);
}

// TODO: Add `ok_〇〇` cases.
