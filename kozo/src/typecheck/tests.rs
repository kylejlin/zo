use crate::{pretty_print::PrettyPrinted, test_utils::*};

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

    insta::assert_display_snapshot!(PrettyPrinted(type_.raw()));
}
