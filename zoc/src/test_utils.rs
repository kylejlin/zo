use crate::{
    check_erasability::{ErasabilityChecker, ErasabilityError},
    eval::{Evaluator, NormalForm, Normalized},
    pretty_print::*,
    syntax_tree::{
        ast::prelude::{spanned_ast::SpanAuxDataFamily, *},
        lexer::lex,
        parser::parse,
        remove_ast_aux_data::AuxDataRemover,
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
    let cst = parse(tokens).unwrap();
    cst.into()
}

pub fn parse_minimal_ast_or_panic(src: &str) -> minimal_ast::Expr {
    let spanned: spanned_ast::Expr = parse_spanned_ast_or_panic(src);
    let mut remover = AuxDataRemover::default();
    remover.convert(spanned)
}

pub fn eval_or_panic(src: &str) -> NormalForm {
    let ast = parse_minimal_ast_or_panic(src);
    Evaluator::default().eval(ast)
}

pub fn get_type_under_empty_tcon_or_panic(src: &str) -> NormalForm {
    let empty = Normalized::<[_; 0]>::new();
    let tcon = LazyTypeContext::Base(empty.as_ref().convert_ref());
    get_type_or_panic(src, tcon)
}

pub fn get_type_or_panic(src: &str, tcon: LazyTypeContext) -> NormalForm {
    let spanned = parse_spanned_ast_or_panic(src);
    TypeChecker::default()
        .get_type(spanned, tcon)
        .pretty_unwrap()
}

pub fn typecheck_and_eval_under_empty_tcon_or_panic(src: &str) -> NormalForm {
    let empty = Normalized::<[_; 0]>::new();
    let tcon = LazyTypeContext::Base(empty.as_ref().convert_ref());
    typecheck_and_eval_or_panic(src, tcon)
}

pub fn typecheck_and_eval_or_panic(src: &str, tcon: LazyTypeContext) -> NormalForm {
    get_type_or_panic(src, tcon);
    eval_or_panic(src)
}

pub fn get_type_error_under_empty_tcon_or_panic(src: &str) -> TypeError<SpanAuxDataFamily> {
    let empty = Normalized::<[_; 0]>::new();
    let tcon = LazyTypeContext::Base(empty.as_ref().convert_ref());
    get_type_error_or_panic(src, tcon)
}

pub fn get_type_error_or_panic(src: &str, tcon: LazyTypeContext) -> TypeError<SpanAuxDataFamily> {
    let spanned = parse_spanned_ast_or_panic(src);
    TypeChecker::default()
        .get_type(spanned, tcon)
        .map(Normalized::into_raw)
        .pretty_unwrap_err()
}

pub fn get_erasability_error_under_empty_tcon_or_panic(src: &str) -> ErasabilityError {
    let empty = Normalized::<[_; 0]>::new();
    let tcon = LazyTypeContext::Base(empty.as_ref().convert_ref());
    get_erasability_error_or_panic(src, tcon)
}

pub fn get_erasability_error_or_panic(src: &str, tcon: LazyTypeContext) -> ErasabilityError {
    let normalized = typecheck_and_eval_or_panic(src, tcon);
    ErasabilityChecker::default()
        .check_erasability_of_well_typed_expr(normalized, tcon)
        .unwrap_err()
}
