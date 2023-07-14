use super::*;

impl TypeChecker {
    pub fn get_type_of_for(
        &mut self,
        for_: RcHashed<cst::For>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let param_type_types =
            self.get_types_of_dependent_expressions(&for_.value.param_types, tcon, scon)?;
        assert_every_expr_is_universe(param_type_types.raw()).map_err(|offender_index| {
            TypeError::UnexpectedNonTypeExpression {
                expr: for_.value.param_types[offender_index].clone(),
                type_: param_type_types.index(offender_index).cloned(),
            }
        })?;

        let param_types_ast = self
            .cst_converter
            .convert_dependent_expressions(for_.value.param_types.clone());
        let normalized_param_types = self.evaluator.eval_dependent_expressions(param_types_ast);
        let tcon_with_param_types = LazyTypeContext::Snoc(
            &tcon,
            normalized_param_types
                .to_derefed()
                .without_digest()
                .derefed(),
        );
        let return_type_type =
            self.get_type(for_.value.return_type.clone(), tcon_with_param_types, scon)?;
        let return_type_type_universe_level = match return_type_type.raw() {
            ast::Expr::Universe(universe_node) => universe_node.value.level,

            _ => {
                return Err(TypeError::UnexpectedNonTypeExpression {
                    expr: for_.value.return_type.clone(),
                    type_: return_type_type,
                })
            }
        };

        let max_level = return_type_type_universe_level
            .max_or_self(get_max_universe_level(param_type_types.raw()));
        Ok(Normalized::universe(ast::UniverseNode { level: max_level }))
    }
}
