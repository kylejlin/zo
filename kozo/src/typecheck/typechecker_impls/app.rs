use super::*;

impl TypeChecker {
    pub fn get_type_of_app(
        &mut self,
        app: RcHashed<cst::App>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let callee_type = self
            .get_type(app.value.callee.clone(), tcon, scon)?
            .try_into_for()
            .map_err(|original| TypeError::CalleeTypeIsNotAForExpression {
                app: app.value.clone(),
                callee_type: original,
            })?;

        let arg_count = app.value.args.len();
        let param_count = callee_type.raw().value.param_types.value.len();
        if arg_count != param_count {
            return Err(TypeError::WrongNumberOfAppArguments {
                app: app.value.clone(),
                callee_type: callee_type.without_digest().cloned(),
                expected: param_count,
                actual: arg_count,
            });
        }

        let arg_types =
            self.get_types_of_independent_expressions(app.value.args.clone(), tcon, scon)?;
        let args_ast = self
            .cst_converter
            .convert_expressions(app.value.args.clone());
        let normalized_args = self.evaluator.eval_expressions(args_ast);

        let substituted_param_types = self.substitute_param_types(
            callee_type.without_digest().param_types(),
            normalized_args.clone(),
        );
        self.assert_expected_type_equalities_holds_after_applying_scon(
            ExpectedTypeEqualities {
                exprs: app.value.args.to_vec_of_cloned(),
                expected_types: substituted_param_types,
                actual_types: arg_types,
                tcon_len: tcon.len(),
            },
            scon,
        )?;

        let arg_substituter = DebDownshiftSubstituter {
            new_exprs: &normalized_args.raw().value,
        };
        let unnormalized_substituted_return_type =
            arg_substituter.replace_debs(callee_type.raw().value.return_type.clone(), 0);
        let substituted_return_type = self.evaluator.eval(unnormalized_substituted_return_type);
        Ok(substituted_return_type)
    }

    fn substitute_param_types(
        &mut self,
        unsubstituted_param_types: Normalized<RcSemHashed<Box<[ast::Expr]>>>,
        normalized_args: Normalized<RcSemHashed<Box<[ast::Expr]>>>,
    ) -> Normalized<Vec<ast::Expr>> {
        let len = normalized_args.raw().value.len();

        let out: Vec<NormalForm> = (0..len)
            .map(|param_index| {
                let unsubstituted_param_type = unsubstituted_param_types
                    .without_digest()
                    .derefed()
                    .index(param_index)
                    .cloned();
                let substituter = DebDownshiftSubstituter {
                    new_exprs: &normalized_args.raw().value[0..param_index],
                };
                let substituted = substituter.replace_debs(unsubstituted_param_type.into_raw(), 0);
                self.evaluator.eval(substituted)
            })
            .collect();

        Normalized::from_vec_normalized(out)
    }
}
