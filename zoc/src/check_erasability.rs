use crate::{
    eval::{NormalForm, Normalized},
    syntax_tree::ast::prelude::minimal_ast::*,
    typecheck::{LazyTypeContext, TypeChecker, TypeError},
};

pub struct ErasabilityChecker {
    pub typechecker: TypeChecker,
}

#[derive(Clone)]
pub enum ErasabilityError {}

impl ErasabilityChecker {
    pub fn check_erasability_of_well_typed_expr(
        &mut self,
        expr: NormalForm,
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        self.check(expr.into_raw(), tcon)
    }
}

trait ExpectWellTyped {
    type Output;

    /// A thin wrapper around `Result::expect`
    /// that lets us omit the panic message.
    fn expect_well_typed(self) -> Self::Output;
}

impl<T, A> ExpectWellTyped for Result<T, TypeError<A>>
where
    A: AuxDataFamily,
    TypeError<A>: std::fmt::Debug,
{
    type Output = T;

    fn expect_well_typed(self) -> Self::Output {
        self.expect("expression should be well_typed")
    }
}

/// The following methods all assume that `expr` is well-typed.
impl ErasabilityChecker {
    fn check(&mut self, expr: Expr, tcon: LazyTypeContext) -> Result<(), ErasabilityError> {
        // If `expr`'s type type is erasable,
        // then it doesn't matter whether `expr` has
        // prop-derived-set instance instances,
        // since `expr` can be erased anyway.
        if self
            .is_type_type_erasable(expr.clone(), tcon)
            .expect_well_typed()
        {
            return Ok(());
        }

        match expr {
            Expr::Match(e) => self.check_match(e, tcon),
            Expr::Fun(e) => self.check_fun(e, tcon),
            Expr::App(e) => self.check_app(e, tcon),
            Expr::For(e) => self.check_for(e, tcon),
            Expr::Ind(_) | Expr::Vcon(_) | Expr::Deb(_) | Expr::Universe(_) => Ok(()),
        }
    }

    fn check_match(
        &mut self,
        match_: RcHashed<Match>,
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        todo!()
    }

    fn check_fun(
        &mut self,
        fun: RcHashed<Fun>,
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        todo!()
    }

    fn check_app(
        &mut self,
        app: RcHashed<App>,
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        todo!()
    }

    fn check_for(
        &mut self,
        for_: RcHashed<For>,
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        todo!()
    }

    fn check_dependent_exprs(
        &mut self,
        exprs: &[Expr],
        tcon: LazyTypeContext,
    ) -> Result<Normalized<Vec<minimal_ast::Expr>>, ErasabilityError> {
        // TODO: Fix

        let mut out: Normalized<Vec<minimal_ast::Expr>> = Normalized::with_capacity(exprs.len());
        let mut normalized_visited_exprs: Normalized<Vec<minimal_ast::Expr>> =
            Normalized::with_capacity(exprs.len());

        for expr in exprs {
            let current_tcon = LazyTypeContext::Snoc(&tcon, normalized_visited_exprs.to_derefed());

            self.check(expr.clone(), current_tcon)?;

            let type_ = self
                .typechecker
                .get_type(expr.clone(), current_tcon)
                .expect_well_typed();
            out.push(type_);

            let expr_minimal = self.typechecker.aux_remover.convert(expr.clone());
            let normalized = self.typechecker.evaluator.eval(expr_minimal);
            normalized_visited_exprs.push(normalized);
        }

        Ok(out)
    }

    fn check_independent_exprs(
        &mut self,
        exprs: &[Expr],
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        for expr in exprs {
            self.check(expr.clone(), tcon)?;
        }

        Ok(())
    }
}

impl ErasabilityChecker {
    fn is_type_type_erasable(
        &mut self,
        expr: Expr,
        tcon: LazyTypeContext,
    ) -> Result<bool, TypeError<UnitAuxDataFamily>> {
        let type_ = self.typechecker.get_type(expr, tcon)?;
        let type_type = self.typechecker.get_type(type_.into_raw(), tcon)?;
        let type_type = type_type
            .try_into_universe()
            .expect("for every expr `t`, `type(type(t))` should be a universe");
        Ok(type_type.raw().hashee.universe.erasable)
    }
}
