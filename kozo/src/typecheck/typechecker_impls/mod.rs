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
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        match expr {
            cst::Expr::Ind(e) => self.get_type_of_ind(e, tcon, scon),
            cst::Expr::Vcon(e) => self.get_type_of_vcon(e, tcon, scon),
            cst::Expr::Match(e) => self.get_type_of_match(e, tcon, scon),
            cst::Expr::Fun(e) => self.get_type_of_fun(e, tcon, scon),
            cst::Expr::App(e) => self.get_type_of_app(e, tcon, scon),
            cst::Expr::For(e) => self.get_type_of_for(e, tcon, scon),
            cst::Expr::Deb(e) => self.get_type_of_deb(e, tcon),
            cst::Expr::Universe(e) => self.get_type_of_universe(e),
        }
    }

    fn get_types_of_dependent_expressions(
        &mut self,
        exprs: &cst::ZeroOrMoreExprs,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<Vec<ast::Expr>>, TypeError> {
        let mut out: Normalized<Vec<ast::Expr>> = Normalized::with_capacity(exprs.len());
        let mut normalized_visited_exprs: Normalized<Vec<ast::Expr>> =
            Normalized::with_capacity(exprs.len());

        for expr in exprs.to_vec() {
            let current_tcon = LazyTypeContext::Snoc(&tcon, normalized_visited_exprs.to_derefed());
            let type_ = self.get_type(expr.clone(), current_tcon, scon)?;
            out.push(type_);

            let expr_ast = self.cst_converter.convert(expr.clone());
            let normalized = self.evaluator.eval(expr_ast);
            normalized_visited_exprs.push(normalized);
        }

        Ok(out)
    }

    fn get_types_of_independent_expressions(
        &mut self,
        exprs: &cst::ZeroOrMoreExprs,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<Vec<ast::Expr>>, TypeError> {
        let mut out: Normalized<Vec<ast::Expr>> = Normalized::with_capacity(exprs.len());

        for expr in exprs.to_vec() {
            let type_ = self.get_type(expr.clone(), tcon, scon)?;
            out.push(type_);
        }

        Ok(out)
    }

    fn typecheck_and_normalize_param_types_with_limit(
        &mut self,
        exprs: &cst::ZeroOrMoreExprs,
        limiting_ind: RcHashed<cst::Ind>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<Normalized<Vec<ast::Expr>>, TypeError> {
        let inclusive_max = UniverseLevel(limiting_ind.hashee.type_.level);
        let param_type_types = self.get_types_of_dependent_expressions(exprs, tcon, scon)?;

        for i in 0..param_type_types.raw().len() {
            let param_type_type: Normalized<&ast::Expr> = param_type_types.index_ref(i);
            let param_type_type_ul = match param_type_type.into_raw() {
                ast::Expr::Universe(universe) => universe.hashee.level,
                _ => {
                    return Err(TypeError::UnexpectedNonTypeExpression {
                        expr: exprs[i].clone(),
                        type_: param_type_type.cloned(),
                    })
                }
            };

            if param_type_type_ul > inclusive_max {
                return Err(TypeError::UniverseInconsistencyInIndDef {
                    index_or_param_type: exprs[i].clone(),
                    level: param_type_type_ul,
                    ind: limiting_ind.hashee.clone(),
                });
            }
        }

        let exprs_ast = self.cst_converter.convert_expressions(exprs.clone());
        let normalized = self.evaluator.eval_expressions(exprs_ast);
        Ok(normalized.to_hashee().cloned())
    }

    fn assert_expr_type_is_universe_and_then_eval(
        &mut self,
        expr: cst::Expr,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let type_ = self.get_type(expr.clone(), tcon, scon)?;

        if !type_.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression { expr, type_ });
        }

        let expr_ast = self.cst_converter.convert(expr.clone());
        let normalized = self.evaluator.eval(expr_ast);
        Ok(normalized)
    }
}
