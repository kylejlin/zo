use ipist::ByteIndex;

use crate::{
    eval::{Evaluator, NormalForm, Normalized},
    pretty_print::*,
    syntax_tree::{
        ast, ipist, ipist_to_ast::IpistToAstConverter, lexer::lex, ost::Span, parser::parse,
    },
    typecheck::{LazySubstitutionContext, LazyTypeContext, TypeChecker},
};

pub fn substitute_with_compounding<'a>(
    iter: impl IntoIterator<Item = (&'a str, &'a str)>,
    last: &'a str,
) -> String {
    let mut replacements = vec![];
    for (from, unreplaced_to) in iter {
        let to = substitute_without_compounding(&replacements, unreplaced_to);
        replacements.push((from, to));
    }
    substitute_without_compounding(&replacements, last)
}

pub fn substitute_without_compounding(replacements: &[(&str, String)], original: &str) -> String {
    let mut result = original.to_string();
    for (from, to) in replacements {
        result = result.replace(from, to);
    }
    result
}

pub fn parse_rch_cst_or_panic(src: &str) -> ipist::Expr {
    let tokens = lex(src).unwrap();
    let nh_cst = parse(tokens).unwrap();
    nh_cst.into()
}

pub fn parse_ast_or_panic(src: &str) -> ast::Expr {
    let rch_cst: ipist::Expr = parse_rch_cst_or_panic(src);
    let mut converter = IpistToAstConverter::default();
    converter.convert(rch_cst)
}

pub fn eval_or_panic(src: &str) -> NormalForm {
    let ast = parse_ast_or_panic(src);
    Evaluator::default().eval(ast)
}

pub fn get_type_under_empty_tcon_and_scon_or_panic(src: &str) -> NormalForm {
    let cst = parse_rch_cst_or_panic(src);
    let empty = Normalized::<[_; 0]>::new();
    TypeChecker::default()
        .get_type(
            cst,
            LazyTypeContext::Base(empty.as_ref().convert_ref()),
            LazySubstitutionContext::Base(&[]),
        )
        .pretty_unwrap()
}

impl ipist::Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Ind(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::Vcon(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::Match(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::Fun(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::App(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::For(e) => (e.hashee.lparen, e.hashee.rparen),
            Self::Deb(e) => e.hashee.span,
            Self::Universe(e) => (
                e.hashee.start,
                ByteIndex(e.hashee.start.0 + "Type".len() + get_digit_count(e.hashee.level)),
            ),
        }
    }
}

fn get_digit_count(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _0_has_1_digit() {
        assert_eq!(1, get_digit_count(0));
    }

    #[test]
    fn _1_has_1_digit() {
        assert_eq!(1, get_digit_count(1));
    }

    #[test]
    fn _9_has_1_digit() {
        assert_eq!(1, get_digit_count(9));
    }

    #[test]
    fn _10_has_2_digits() {
        assert_eq!(2, get_digit_count(10));
    }

    #[test]
    fn _11_has_2_digits() {
        assert_eq!(2, get_digit_count(11));
    }

    #[test]
    fn _19_has_2_digits() {
        assert_eq!(2, get_digit_count(19));
    }

    #[test]
    fn _99_has_2_digits() {
        assert_eq!(2, get_digit_count(99));
    }

    #[test]
    fn _100_has_3_digits() {
        assert_eq!(3, get_digit_count(100));
    }

    #[test]
    fn _101_has_3_digits() {
        assert_eq!(3, get_digit_count(101));
    }

    #[test]
    fn _999_has_3_digits() {
        assert_eq!(3, get_digit_count(999));
    }

    #[test]
    fn _1000_has_4_digits() {
        assert_eq!(4, get_digit_count(1000));
    }

    #[test]
    fn _1001_has_4_digits() {
        assert_eq!(4, get_digit_count(1001));
    }

    #[test]
    fn _9999_has_4_digits() {
        assert_eq!(4, get_digit_count(9999));
    }
}
