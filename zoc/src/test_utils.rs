use crate::{
    eval::{Evaluator, NormalForm, Normalized},
    pretty_print::*,
    syntax_tree::{
        lexer::lex, minimal_ast, parser::parse, spanned_ast, spanned_ast_to_minimal::SpanRemover,
    },
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

pub fn parse_spanned_ast_or_panic(src: &str) -> spanned_ast::Expr {
    let tokens = lex(src).unwrap();
    let ost = parse(tokens).unwrap();
    ost.into()
}

pub fn parse_minimal_ast_or_panic(src: &str) -> minimal_ast::Expr {
    let spanned: spanned_ast::Expr = parse_spanned_ast_or_panic(src);
    let mut converter = SpanRemover::default();
    converter.convert(spanned)
}

pub fn eval_or_panic(src: &str) -> NormalForm {
    let ast = parse_minimal_ast_or_panic(src);
    Evaluator::default().eval(ast)
}

pub fn get_type_under_empty_tcon_or_panic(src: &str) -> NormalForm {
    let spanned = parse_spanned_ast_or_panic(src);
    let empty = Normalized::<[_; 0]>::new();
    TypeChecker::default()
        .get_type(spanned, LazyTypeContext::Base(empty.as_ref().convert_ref()))
        .pretty_unwrap()
}

pub fn get_type_error_under_empty_tcon_or_panic(src: &str) -> TypeError {
    let empty = Normalized::<[_; 0]>::new();
    let tcon = LazyTypeContext::Base(empty.as_ref().convert_ref());
    get_type_error_or_panic(src, tcon)
}

pub fn get_type_error_or_panic(src: &str, tcon: LazyTypeContext) -> TypeError {
    let spanned = parse_spanned_ast_or_panic(src);
    TypeChecker::default()
        .get_type(spanned, tcon)
        .map(Normalized::into_raw)
        .pretty_unwrap_err()
}
