use super::*;

impl TypeChecker {
    pub fn get_type_of_for(
        &mut self,
        for_g0: RcHashed<cst::For>,
        tcon_g0: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_type_types_g0 =
            self.get_types_of_dependent_expressions(&for_g0.hashee.param_types, tcon_g0, scon)?;

        self.assert_every_type_is_universe(
            param_type_types_g0.to_derefed(),
            &for_g0.hashee.param_types,
        )?;

        let param_types_g0_ast = self
            .cst_converter
            .convert_expressions(&for_g0.hashee.param_types);
        let normalized_param_types_g0 = self.evaluator.eval_expressions(param_types_g0_ast);

        let tcon_with_param_types_g1 =
            LazyTypeContext::Snoc(&tcon_g0, normalized_param_types_g0.to_hashee().derefed());

        let return_type_type_g1 = self.get_type(
            for_g0.hashee.return_type.clone(),
            tcon_with_param_types_g1,
            scon,
        )?;
        let return_type_type_g1_universe_level = match return_type_type_g1.raw() {
            ast::Expr::Universe(universe_node) => universe_node.hashee.level,

            _ => {
                return Err(TypeError::UnexpectedNonTypeExpression {
                    expr: for_g0.hashee.return_type.clone(),
                    type_: return_type_type_g1,
                })
            }
        };

        let max_level = return_type_type_g1_universe_level
            .max_or_self(get_max_universe_level(param_type_types_g0.raw()));
        Ok(Normalized::universe(ast::UniverseNode { level: max_level }))
    }

    fn assert_every_type_is_universe(
        &mut self,
        types: Normalized<&[ast::Expr]>,
        exprs: &[cst::Expr],
    ) -> Result<(), TypeError> {
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
