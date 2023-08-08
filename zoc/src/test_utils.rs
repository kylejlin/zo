use crate::{
    eval::{Evaluator, NormalForm, Normalized},
    pretty_print::*,
    syntax_tree::{ast, ipist, ipist_to_ast::IpistToAstConverter, lexer::lex, parser::parse},
    typecheck::{LazyTypeContext, TypeChecker, TypeError},
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

pub fn get_type_under_empty_tcon_or_panic(src: &str) -> NormalForm {
    let cst = parse_rch_cst_or_panic(src);
    let empty = Normalized::<[_; 0]>::new();
    TypeChecker::default()
        .get_type(cst, LazyTypeContext::Base(empty.as_ref().convert_ref()))
        .pretty_unwrap()
}

pub fn get_type_error_under_empty_tcon_or_panic(src: &str) -> TypeError {
    let cst = parse_rch_cst_or_panic(src);
    let empty = Normalized::<[_; 0]>::new();
    TypeChecker::default()
        .get_type(cst, LazyTypeContext::Base(empty.as_ref().convert_ref()))
        .map(Normalized::into_raw)
        .pretty_unwrap_err()
}
