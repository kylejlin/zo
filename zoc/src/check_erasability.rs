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
        self.check(checkee.hashee.callee.clone(), tcon)?;
        self.check_independent_exprs(&checkee.hashee.args.hashee, tcon)?;
        Ok(())
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

impl ErasabilityChecker {
    fn check_match_erasability_without_checking_children<A: AuxDataFamily>(
        &mut self,
        match_g0: RcHashed<ast::Match<A>>,
        matchee_type_ind_g0: Normalized<RcHashed<minimal_ast::Ind>>,
        match_return_type_type: RcHashed<minimal_ast::UniverseNode>,
        tcon_g0: LazyTypeContext,
    ) -> Result<(), TypeError<A>> {
        if match_return_type_type.hashee.universe.erasable
            || !matchee_type_ind_g0.raw().hashee.universe.erasable
            || self.does_well_typed_ind_have_at_most_one_vcon_def_where_all_params_are_erasable(
                &matchee_type_ind_g0.raw().hashee,
                tcon_g0,
            )
        {
            return Ok(());
        }

        Err(
            TypeError::MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable {
                match_: match_g0.hashee.clone(),
                matchee_type_type: minimal_ast::UniverseNode {
                    universe: matchee_type_ind_g0.raw().hashee.universe,
                    aux_data: (),
                },
                match_return_type_type: match_return_type_type.hashee.clone(),
            },
        )
    }

    fn does_well_typed_ind_have_at_most_one_vcon_def_where_all_params_are_erasable<
        A: AuxDataFamily,
    >(
        &mut self,
        ind_g0: &ast::Ind<A>,
        tcon_g0: LazyTypeContext,
    ) -> bool {
        let vcon_defs = &ind_g0.vcon_defs.hashee;

        if vcon_defs.len() > 1 {
            return false;
        }

        if vcon_defs.len() == 0 {
            return true;
        }

        let index_types_g0_minimal = self
            .typechecker
            .aux_remover
            .convert_expressions(&ind_g0.index_types.hashee);
        let normalized_index_types_g0 = self
            .typechecker
            .evaluator
            .eval_expressions(index_types_g0_minimal);
        let universe_node = NormalForm::universe(minimal_ast::UniverseNode {
            universe: ind_g0.universe,
            aux_data: (),
        });
        let ind_type_g0 =
            Normalized::for_(normalized_index_types_g0, universe_node).collapse_if_nullary();

        let ind_type_singleton = Normalized::<[_; 1]>::new(ind_type_g0.clone());
        let tcon_with_ind_type_g1 =
            LazyTypeContext::Snoc(&tcon_g0, ind_type_singleton.as_ref().convert_ref());

        let vcon_def_g1 = &vcon_defs[0];
        let vcon_def_param_type_types_g1 = self
            .typechecker
            .get_types_of_dependent_expressions(
                &vcon_def_g1.param_types.hashee,
                tcon_with_ind_type_g1,
            )
            .map_err(|err| err.remove_ast_aux_data(&mut self.typechecker.aux_remover))
            .expect("`ind_g0` is should be well-typed");

        vcon_def_param_type_types_g1
            .into_raw()
            .into_iter()
            .all(|param_type| {
                let param_type = param_type.try_into_universe().expect("`ind_g0` is well-typed, so every vcon def param type type should be a universe");
                param_type.hashee.universe.erasable
            })
    }
}
