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
    pub fn get_type<A: AstFamily>(
        &mut self,
        expr: ast::Expr<A>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError<A>> {
        match expr {
            ast::Expr::Ind(e) => self.get_type_of_ind(e, tcon),
            ast::Expr::Vcon(e) => self.get_type_of_vcon(e, tcon),
            ast::Expr::Match(e) => self.get_type_of_match(e, tcon),
            ast::Expr::Fun(e) => self.get_type_of_fun(e, tcon),
            ast::Expr::App(e) => self.get_type_of_app(e, tcon),
            ast::Expr::For(e) => self.get_type_of_for(e, tcon),
            ast::Expr::Deb(e) => self.get_type_of_deb(e, tcon),
            ast::Expr::Universe(e) => self.get_type_of_universe(e),
        }
    }

    fn get_types_of_dependent_expressions<A: AstFamily>(
        &mut self,
        exprs: &[ast::Expr<A>],
        tcon: LazyTypeContext,
    ) -> Result<Normalized<Vec<minimal_ast::Expr>>, TypeError<A>> {
        let mut out: Normalized<Vec<minimal_ast::Expr>> = Normalized::with_capacity(exprs.len());
        let mut normalized_visited_exprs: Normalized<Vec<minimal_ast::Expr>> =
            Normalized::with_capacity(exprs.len());

        for expr in exprs {
            let current_tcon = LazyTypeContext::Snoc(&tcon, normalized_visited_exprs.to_derefed());
            let type_ = self.get_type(expr.clone(), current_tcon)?;
            out.push(type_);

            let expr_minimal = self.aux_remover.convert(expr.clone());
            let normalized = self.evaluator.eval(expr_minimal);
            normalized_visited_exprs.push(normalized);
        }

        Ok(out)
    }

    fn get_types_of_independent_expressions<A: AstFamily>(
        &mut self,
        exprs: &[ast::Expr<A>],
        tcon: LazyTypeContext,
    ) -> Result<Normalized<Vec<minimal_ast::Expr>>, TypeError<A>> {
        let mut out: Normalized<Vec<minimal_ast::Expr>> = Normalized::with_capacity(exprs.len());

        for expr in exprs {
            let type_ = self.get_type(expr.clone(), tcon)?;
            out.push(type_);
        }

        Ok(out)
    }

    fn typecheck_param_types_with_limit_and_normalize<A: AstFamily>(
        &mut self,
        exprs: &[ast::Expr<A>],
        limit: impl UniverseLimit<A>,
        tcon: LazyTypeContext,
    ) -> Result<Normalized<Vec<minimal_ast::Expr>>, TypeError<A>> {
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

            limit.assert_ul_is_within_limit(param_type_type_ul, exprs[i].clone())?;
        }

        let exprs_minimal = self.aux_remover.convert_expressions(exprs.clone());
        let normalized = self.evaluator.eval_expressions(exprs_minimal);
        Ok(normalized.to_hashee().cloned())
    }

    fn assert_expr_type_is_universe<A: AstFamily>(
        &mut self,
        expr: ast::Expr<A>,
        tcon: LazyTypeContext,
    ) -> Result<RcHashed<minimal_ast::UniverseNode>, TypeError<A>> {
        let type_ = self.get_type(expr.clone(), tcon)?;

        match type_.raw() {
            ast::Expr::Universe(universe) => Ok(universe.clone()),

            _ => Err(TypeError::UnexpectedNonTypeExpression { expr, type_ }),
        }
    }

    fn assert_expr_type_is_universe_and_then_eval<A: AstFamily>(
        &mut self,
        expr: ast::Expr<A>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError<A>> {
        let type_ = self.get_type(expr.clone(), tcon)?;

        if !type_.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression { expr, type_ });
        }

        let expr_minimal = self.aux_remover.convert(expr.clone());
        let normalized = self.evaluator.eval(expr_minimal);
        Ok(normalized)
    }
}

trait UniverseLimit<A: AstFamily> {
    fn assert_ul_is_within_limit(
        &self,
        param_type_type_universe: Universe,
        expr: ast::Expr<A>,
    ) -> Result<(), TypeError<A>>;
}

struct LimitToIndUniverse<A: AstFamily>(RcHashed<ast::Ind<A>>);

impl<A: AstFamily> UniverseLimit<A> for LimitToIndUniverse<A> {
    fn assert_ul_is_within_limit(
        &self,
        param_type_type_universe: Universe,
        expr: ast::Expr<A>,
    ) -> Result<(), TypeError<A>> {
        let inclusive_max = self.0.hashee.universe.level;
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

#[derive(Default)]
struct NoLimit<A: AstFamily>(PhantomData<A>);

impl<A: AstFamily> UniverseLimit<A> for NoLimit<A> {
    fn assert_ul_is_within_limit(&self, _: Universe, _: ast::Expr<A>) -> Result<(), TypeError<A>> {
        Ok(())
    }
}

// TODO: Reconsider we need `Normalized<Vec<T>>`.
// It might be easier to use `Vec<Normalized<T>>`.
