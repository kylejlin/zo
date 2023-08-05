use super::*;

use zoc::pretty_print::PrettyPrint;

#[test]
fn two() {
    let src = r#"
ind Nat
    case zero
    case succ(_: Nat)
    return Set0

succ(succ(zero))
"#;
    let cst = parse_or_panic(src);
    let zo = may_to_zo(&cst).unwrap();
    insta::assert_display_snapshot!(PrettyPrint(&zo));
}

fn parse_or_panic(src: &str) -> mnode::Expr {
    let tokens = crate::lexer::lex(src).unwrap();
    crate::parser::parse(tokens).unwrap()
}
