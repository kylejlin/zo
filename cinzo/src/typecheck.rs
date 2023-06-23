use crate::ast::*;

#[derive(Debug, Clone)]
pub enum TypeErr {}

#[derive(Debug, Clone, Copy)]
pub enum LazyContext<'a> {
    Base(&'a [Expr]),
    Cons(&'a [Expr], &'a LazyContext<'a>),
}

pub fn get_type(expr: Expr, context: LazyContext) -> Result<Expr, TypeErr> {
    todo!()
}
