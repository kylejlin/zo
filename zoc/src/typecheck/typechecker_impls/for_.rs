use super::*;

impl TypeChecker {
    pub fn get_type_of_for<A: AuxDataFamily>(
        &mut self,
        for_g0: RcHashed<ast::For<A>>,
        tcon_g0: LazyTypeContext,
    ) -> Result<NormalForm, TypeError<A>> {
        self.assert_for_has_at_least_one_param(for_g0.clone())?;

        let param_type_types_g0 =
            self.get_types_of_dependent_expressions(&for_g0.hashee.param_types.hashee, tcon_g0)?;

        self.assert_every_type_is_universe(
            param_type_types_g0.to_derefed(),
            &for_g0.hashee.param_types.hashee,
        )?;

        let param_types_g0_ast = self
            .span_remover
            .convert_expressions(&for_g0.hashee.param_types.hashee);
        let normalized_param_types_g0 = self.evaluator.eval_expressions(param_types_g0_ast);

        let tcon_with_param_types_g1 =
            LazyTypeContext::Snoc(&tcon_g0, normalized_param_types_g0.to_hashee().derefed());

        let return_type_type_g1 =
            self.get_type(for_g0.hashee.return_type.clone(), tcon_with_param_types_g1)?;
        let return_type_type_g1_universe_level = match return_type_type_g1.raw() {
            minimal_ast::Expr::Universe(universe_node) => universe_node.hashee.universe,

            _ => {
                return Err(TypeError::UnexpectedNonTypeExpression {
                    expr: for_g0.hashee.return_type.clone(),
                    type_: return_type_type_g1,
                })
            }
        };

        let max_level = return_type_type_g1_universe_level
            .level
            .max_or_self(get_max_universe_level(param_type_types_g0.raw()));
        Ok(Normalized::universe(minimal_ast::UniverseNode {
            universe: Universe {
                level: max_level,
                erasable: return_type_type_g1_universe_level.erasable,
            },
            aux_data: (),
        }))
    }

    fn assert_for_has_at_least_one_param<A: AuxDataFamily>(
        &mut self,
        for_: RcHashed<ast::For<A>>,
    ) -> Result<(), TypeError<A>> {
        if for_.hashee.param_types.hashee.is_empty() {
            return Err(TypeError::ForHasZeroParams {
                for_: for_.hashee.clone(),
            });
        }

        Ok(())
    }

    fn assert_every_type_is_universe<A: AuxDataFamily>(
        &mut self,
        types: Normalized<&[minimal_ast::Expr]>,
        exprs: &[ast::Expr<A>],
    ) -> Result<(), TypeError<A>> {
        for i in 0..types.raw().len() {
            if !types.raw()[i].is_universe() {
                return Err(TypeError::UnexpectedNonTypeExpression {
                    expr: exprs[i].clone(),
                    type_: types.index_ref(i).cloned(),
                });
            }
        }

        Ok(())
    }
}
