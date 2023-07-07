use crate::{
    eval::{Evaluator, NormalForm, Normalized},
    syntax_tree::{ast, lexer::lex, parser::parse, rch_cst, rch_cst_to_ast::RchCstToAstConverter},
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

pub fn parse_rch_cst_or_panic(src: &str) -> rch_cst::Expr {
    let tokens = lex(src).unwrap();
    let nh_cst = parse(tokens).unwrap();
    nh_cst.into()
}

pub fn parse_ast_or_panic(src: &str) -> ast::Expr {
    let rch_cst: rch_cst::Expr = parse_rch_cst_or_panic(src);
    let mut converter = RchCstToAstConverter::default();
    converter.convert(rch_cst)
}

pub fn eval_or_panic(src: &str) -> NormalForm {
    let ast = parse_ast_or_panic(src);
    Evaluator::default().eval(ast)
}

pub fn get_type_under_empty_tcon_and_scon_or_panic(src: &str) -> NormalForm {
    let cst = parse_rch_cst_or_panic(src);
    TypeChecker::default()
        .get_type(
            cst,
            LazyTypeContext::Base(Normalized::empty_static()),
            LazySubstitutionContext::Base(&[]),
        )
        .unwrap()
}
