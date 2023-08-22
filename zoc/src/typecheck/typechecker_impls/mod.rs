use super::*;

mod app;
mod deb_node;
mod for_;
mod fun;
mod ind;
mod match_;
mod universe_node;
mod vcon;

impl TypeChecker {
    pub fn get_type(
        &mut self,
        expr: cst::Expr,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        match expr {
            cst::Expr::Ind(e) => self.get_type_of_ind(e, tcon),
            cst::Expr::Vcon(e) => self.get_type_of_vcon(e, tcon),
            cst::Expr::Match(e) => self.get_type_of_match(e, tcon),
            cst::Expr::Fun(e) => self.get_type_of_fun(e, tcon),
            cst::Expr::App(e) => self.get_type_of_app(e, tcon),
            cst::Expr::For(e) => self.get_type_of_for(e, tcon),
            cst::Expr::Deb(e) => self.get_type_of_deb(e, tcon),
            cst::Expr::Universe(e) => self.get_type_of_universe(e),
        }
    }

    fn get_types_of_dependent_expressions(
        &mut self,
        exprs: &[cst::Expr],
        tcon: LazyTypeContext,
    ) -> Result<Normalized<Vec<minimal_ast::Expr>>, TypeError> {
        let mut out: Normalized<Vec<minimal_ast::Expr>> = Normalized::with_capacity(exprs.len());
        let mut normalized_visited_exprs: Normalized<Vec<minimal_ast::Expr>> =
            Normalized::with_capacity(exprs.len());

        for expr in exprs {
            let current_tcon = LazyTypeContext::Snoc(&tcon, normalized_visited_exprs.to_derefed());
            let type_ = self.get_type(expr.clone(), current_tcon)?;
            out.push(type_);

            let expr_ast = self.cst_converter.convert(expr.clone());
            let normalized = self.evaluator.eval(expr_ast);
            normalized_visited_exprs.push(normalized);
        }

        Ok(out)
    }

    fn get_types_of_independent_expressions(
        &mut self,
        exprs: &[cst::Expr],
        tcon: LazyTypeContext,
    ) -> Result<Normalized<Vec<minimal_ast::Expr>>, TypeError> {
        let mut out: Normalized<Vec<minimal_ast::Expr>> = Normalized::with_capacity(exprs.len());

        for expr in exprs {
            let type_ = self.get_type(expr.clone(), tcon)?;
            out.push(type_);
        }

        Ok(out)
    }

    fn typecheck_and_normalize_param_types_with_limit(
        &mut self,
        exprs: &[cst::Expr],
        limiter: impl UniverseLimit,
        tcon: LazyTypeContext,
    ) -> Result<Normalized<Vec<minimal_ast::Expr>>, TypeError> {
        let param_type_types = self.get_types_of_dependent_expressions(exprs, tcon)?;

        for i in 0..param_type_types.raw().len() {
            let param_type_type: Normalized<&minimal_ast::Expr> = param_type_types.index_ref(i);
            let param_type_type_ul = match param_type_type.into_raw() {
                minimal_ast::Expr::Universe(universe) => universe.hashee.universe,
                _ => {
                    return Err(TypeError::UnexpectedNonTypeExpression {
                        expr: exprs[i].clone(),
                        type_: param_type_type.cloned(),
                    })
                }
            };

            limiter.assert_ul_is_within_limit(param_type_type_ul, exprs[i].clone())?;
        }

        let exprs_ast = self.cst_converter.convert_expressions(exprs.clone());
        let normalized = self.evaluator.eval_expressions(exprs_ast);
        Ok(normalized.to_hashee().cloned())
    }

    fn assert_expr_type_is_universe(
        &mut self,
        expr: cst::Expr,
        tcon: LazyTypeContext,
    ) -> Result<(), TypeError> {
        let type_ = self.get_type(expr.clone(), tcon)?;

        if !type_.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression { expr, type_ });
        }

        Ok(())
    }

    fn assert_expr_type_is_universe_and_then_eval(
        &mut self,
        expr: cst::Expr,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        let type_ = self.get_type(expr.clone(), tcon)?;

        if !type_.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression { expr, type_ });
        }

        let expr_ast = self.cst_converter.convert(expr.clone());
        let normalized = self.evaluator.eval(expr_ast);
        Ok(normalized)
    }
}

trait UniverseLimit {
    fn assert_ul_is_within_limit(
        &self,
        param_type_type_universe: Universe,
        expr: cst::Expr,
    ) -> Result<(), TypeError>;
}

struct LimitToIndUniverse(RcHashed<cst::Ind>);

impl UniverseLimit for LimitToIndUniverse {
    fn assert_ul_is_within_limit(
        &self,
        param_type_type_universe: Universe,
        expr: cst::Expr,
    ) -> Result<(), TypeError> {
        let inclusive_max = UniverseLevel(self.0.hashee.type_.level);
        if param_type_type_universe.level > inclusive_max {
            return Err(TypeError::UniverseInconsistencyInIndDef {
                index_or_param_type: expr.clone(),
                universe: param_type_type_universe,
                ind: self.0.hashee.clone(),
            });
        }

        Ok(())
    }
}

struct NoLimit;

impl UniverseLimit for NoLimit {
    fn assert_ul_is_within_limit(&self, _: Universe, _: cst::Expr) -> Result<(), TypeError> {
        Ok(())
    }
}
