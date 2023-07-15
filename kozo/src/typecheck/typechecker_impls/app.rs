use super::*;

impl TypeChecker {
    pub fn get_type_of_app(
        &mut self,
        app: RcHashed<cst::App>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let callee_type = self
            .get_type(app.hashee.callee.clone(), tcon, scon)?
            // TODO: Check the scon case.
            .try_into_for()
            .map_err(|original| TypeError::CalleeTypeIsNotAForExpression {
                app: app.hashee.clone(),
                callee_type: original,
            })?;

        let arg_count = app.hashee.args.len();
        let param_count = callee_type.raw().hashee.param_types.hashee.len();
        if arg_count != param_count {
            return Err(TypeError::WrongNumberOfAppArguments {
                app: app.hashee.clone(),
                callee_type: callee_type.to_hashee().cloned(),
                expected: param_count,
                actual: arg_count,
            });
        }

        let arg_types = self.get_types_of_independent_expressions(&app.hashee.args, tcon, scon)?;
        let args_ast = self
            .cst_converter
            .convert_expressions(app.hashee.args.clone());
        let normalized_args = self.evaluator.eval_expressions(args_ast);

        let substituted_param_types = self.substitute_param_types(
            callee_type.to_hashee().param_types().cloned(),
            normalized_args.clone(),
        );
        self.assert_expected_type_equalities_holds_after_applying_scon(
            ExpectedTypeEqualities {
                exprs: app.hashee.args.to_vec_of_cloned(),
                expected_types: substituted_param_types,
                actual_types: arg_types,
                tcon_len: tcon.len(),
            },
            scon,
        )?;

        let arg_substituter = DebDownshiftSubstituter {
            new_exprs: &normalized_args.raw().hashee,
        };
        let unnormalized_substituted_return_type = callee_type
            .raw()
            .hashee
            .return_type
            .clone()
            .replace_debs(&arg_substituter, 0);
        let substituted_return_type = self.evaluator.eval(unnormalized_substituted_return_type);
        Ok(substituted_return_type)
    }

    fn substitute_param_types(
        &mut self,
        unsubstituted_param_types: Normalized<RcSemHashedVec<ast::Expr>>,
        normalized_args: Normalized<RcSemHashedVec<ast::Expr>>,
    ) -> Normalized<Vec<ast::Expr>> {
        let len = normalized_args.raw().hashee.len();

        (0..len)
            .map(|param_index| {
                let unsubstituted_param_type = unsubstituted_param_types
                    .to_hashee()
                    .derefed()
                    .index_ref(param_index)
                    .cloned();
                let substituter = DebDownshiftSubstituter {
                    new_exprs: &normalized_args.raw().hashee[0..param_index],
                };
                let substituted = unsubstituted_param_type
                    .into_raw()
                    .replace_debs(&substituter, 0);
                self.evaluator.eval(substituted)
            })
            .collect()
    }
}
