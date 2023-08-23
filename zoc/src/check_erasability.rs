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
        match expr {
            Expr::Ind(e) => self.check_ind(e, tcon),
            Expr::Vcon(e) => self.check_vcon(e, tcon),
            Expr::Match(e) => self.check_match(e, tcon),
            Expr::Fun(e) => self.check_fun(e, tcon),
            Expr::App(e) => self.check_app(e, tcon),
            Expr::For(e) => self.check_for(e, tcon),
            Expr::Deb(_) | Expr::Universe(_) => Ok(()),
        }
    }

    fn check_ind(
        &mut self,
        ind: RcHashed<Ind>,
        tcon_g0: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        self.check_dependent_exprs(&ind.hashee.index_types.hashee, tcon_g0)?;

        let ind_type_g0 = self
            .typechecker
            .get_type_of_ind(ind.clone(), tcon_g0)
            .expect_well_typed();
        let ind_singleton = Normalized::<[Expr; 1]>::new(ind_type_g0);
        let tcon_with_ind_type_g1 =
            LazyTypeContext::Snoc(&tcon_g0, ind_singleton.as_ref().convert_ref());

        self.check_vcon_defs(&ind.hashee.vcon_defs.hashee, tcon_with_ind_type_g1)?;

        Ok(())
    }

    fn check_vcon_defs(
        &mut self,
        defs: &[VconDef],
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        for def in defs {
            self.check_vcon_def(def, tcon)?;
        }
        Ok(())
    }

    fn check_vcon_def(
        &mut self,
        def: &VconDef,
        tcon_g0: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        let param_types = self.check_dependent_exprs(&def.param_types.hashee, tcon_g0)?;
        let tcon_with_param_types_g1 = LazyTypeContext::Snoc(&tcon_g0, param_types.to_derefed());

        self.check_independent_exprs(&def.index_args.hashee, tcon_with_param_types_g1)?;

        Ok(())
    }

    fn check_vcon(
        &mut self,
        vcon: RcHashed<Vcon>,
        tcon: LazyTypeContext,
    ) -> Result<(), ErasabilityError> {
        todo!()
    }

    fn check_match(
        &mut self,
        r#match: RcHashed<Match>,
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
