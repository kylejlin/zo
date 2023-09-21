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

/// The following methods all assume that `checkee` is well-typed.
impl ErasabilityChecker {
    fn check(&mut self, checkee: Expr, tcon: LazyTypeContext) -> Result<(), ErasabilityError> {
        // If `expr`'s type type is erasable,
        // then it doesn't matter whether `expr` has
        // prop-derived-set instance instances,
        // since `expr` can be erased anyway.
        if self
            .is_type_type_erasable(checkee.clone(), tcon)
            .expect_well_typed()
        {
            return Ok(());
        }

        match checkee {
            Expr::Match(c) => self.check_match(c, tcon),
            Expr::Fun(c) => self.check_fun(c, tcon),
            Expr::App(c) => self.check_app(c, tcon),

            // - We can skip checking `ind`s since we can erase them entirely.
            //
            // - We can skip checking `vcon`s since we can
            //   almost erase them--we can erase all but their `vcon_index`.
            //   The `vcon_index` is a static value, so we don't need to
            //   worry about it depending on an erasable value.
            //
            // - We can skip checking `for`s since we can erase them entirely.
            //
            // - We can skip checking debs, since they are their only dependency.
            //   So, they obviously cannot simultaneously
            //   depend on an erasable value while also producing
            //   a non-erasable output.
            //
            // - We can skip checking universes since we can erase them entirely.
            Expr::Ind(_) | Expr::Vcon(_) | Expr::For(_) | Expr::Deb(_) | Expr::Universe(_) => {
                Ok(())
            }
        }
    }

    fn check_match(
        &mut self,
        checkee: RcHashed<Match>,
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        todo!()
    }

    fn check_fun(
        &mut self,
        checkee: RcHashed<Fun>,
        tcon_g0: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        // We can skip checking the param types,
        // since each of their type types (i.e., each param type type type)
        // equals some Prop.

        // We can skip checking the return type for the same reason.

        // The only child we must check is the fun's `return_val`.

        let param_types_g0 = self
            .typechecker
            .evaluator
            .eval_expressions(checkee.hashee.param_types.clone());
        let tcon_with_params_g1 =
            LazyTypeContext::Snoc(&tcon_g0, param_types_g0.to_hashee().convert_ref());
        let param_count = checkee.hashee.param_types.hashee.len();

        let recursive_fun_param_type_g1 = self
            .typechecker
            .get_type_of_fun(checkee.clone(), tcon_g0)
            .expect_well_typed()
            .upshift(param_count, 0);
        let singleton_g1 = Normalized::<[_; 1]>::new(recursive_fun_param_type_g1);
        let tcon_with_params_and_recursive_fun_param_g2 =
            LazyTypeContext::Snoc(&tcon_with_params_g1, singleton_g1.as_ref().convert_ref());

        self.check(
            checkee.hashee.return_val.clone(),
            tcon_with_params_and_recursive_fun_param_g2,
        )
    }

    fn check_app(
        &mut self,
        checkee: RcHashed<App>,
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        todo!()
    }

    fn check_dependent_exprs(
        &mut self,
        checkee: &[Expr],
        tcon: LazyTypeContext,
    ) -> Result<Normalized<Vec<minimal_ast::Expr>>, ErasabilityError> {
        // TODO: Fix

        let mut out: Normalized<Vec<minimal_ast::Expr>> = Normalized::with_capacity(checkee.len());
        let mut normalized_visited_exprs: Normalized<Vec<minimal_ast::Expr>> =
            Normalized::with_capacity(checkee.len());

        for expr in checkee {
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
        checkee: &[Expr],
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        for expr in checkee {
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
