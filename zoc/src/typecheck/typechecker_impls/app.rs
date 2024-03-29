use super::*;

impl TypeChecker {
    pub fn get_type_of_app<A: AuxDataFamily>(
        &mut self,
        app: RcHashed<ast::App<A>>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError<A>> {
        self.assert_app_has_at_least_one_arg(app.clone())?;

        let callee_type = self.get_type(app.hashee.callee.clone(), tcon)?;
        let callee_type = self.assert_callee_type_is_a_for_expression(callee_type, app.clone())?;

        let callee_type_param_types = callee_type.to_hashee().param_types().cloned();
        let callee_type_return_type_g0f = callee_type.to_hashee().return_type().cloned();

        self.assert_arg_count_is_correct(app.clone(), callee_type.clone())?;

        let arg_types = self.get_types_of_independent_expressions(&app.hashee.args.hashee, tcon)?;

        let args_minimal = self
            .aux_remover
            .convert_expressions(&app.hashee.args.hashee);
        let normalized_args = self.evaluator.eval_expressions(args_minimal);

        let substituted_callee_type_param_types = self.substitute_callee_type_param_types(
            callee_type_param_types.clone(),
            normalized_args.clone(),
        );

        self.assert_expected_type_equalities_holds(ExpectedTypeEqualities {
            exprs: &app.hashee.args.hashee,
            expected_types: substituted_callee_type_param_types.to_hashee().derefed(),
            actual_types: arg_types.to_derefed(),
        })?;

        let substituted_callee_type_return_type =
            self.substitute_callee_type_return_type(callee_type_return_type_g0f, normalized_args);
        Ok(substituted_callee_type_return_type)
    }

    fn assert_app_has_at_least_one_arg<A: AuxDataFamily>(
        &mut self,
        app: RcHashed<ast::App<A>>,
    ) -> Result<(), TypeError<A>> {
        if app.hashee.args.hashee.is_empty() {
            return Err(TypeError::AppHasZeroArgs {
                app: app.hashee.clone(),
            });
        }

        Ok(())
    }

    fn assert_callee_type_is_a_for_expression<A: AuxDataFamily>(
        &mut self,
        callee_type: NormalForm,
        app: RcHashed<ast::App<A>>,
    ) -> Result<Normalized<RcHashed<minimal_ast::For>>, TypeError<A>> {
        if let Ok(for_) = callee_type.clone().try_into_for() {
            return Ok(for_);
        }

        Err(TypeError::CalleeTypeIsNotAForExpression {
            app: app.hashee.clone(),
            callee_type,
        })
    }

    fn assert_arg_count_is_correct<A: AuxDataFamily>(
        &mut self,
        app: RcHashed<ast::App<A>>,
        callee_type: Normalized<RcHashed<minimal_ast::For>>,
    ) -> Result<(), TypeError<A>> {
        let arg_count = app.hashee.args.hashee.len();
        let param_count = callee_type.raw().hashee.param_types.hashee.len();
        if arg_count != param_count {
            return Err(TypeError::WrongNumberOfAppArguments {
                app: app.hashee.clone(),
                callee_type: callee_type.to_hashee().cloned(),
                expected: param_count,
                actual: arg_count,
            });
        }

        Ok(())
    }

    pub(in crate::typecheck) fn substitute_callee_type_param_types(
        &mut self,
        param_types: Normalized<RcHashedVec<minimal_ast::Expr>>,
        args: Normalized<RcHashedVec<minimal_ast::Expr>>,
    ) -> Normalized<RcHashedVec<minimal_ast::Expr>> {
        let len = args.raw().hashee.len();

        (0..len)
            .map(|param_index| {
                let unsubstituted_param_type = param_types
                    .to_hashee()
                    .derefed()
                    .index_ref(param_index)
                    .cloned();
                let substituter = DebDownshiftSubstituter {
                    new_exprs: &args.raw().hashee[0..param_index],
                };
                let substituted = unsubstituted_param_type
                    .into_raw()
                    .replace_debs(&substituter, 0);
                self.evaluator.eval(substituted)
            })
            .collect::<Normalized<Vec<_>>>()
            .into_rc_hashed()
    }

    fn substitute_callee_type_return_type(
        &mut self,
        return_type_g0f: NormalForm,
        args: Normalized<RcHashedVec<minimal_ast::Expr>>,
    ) -> NormalForm {
        let substituter = DebDownshiftSubstituter {
            new_exprs: &args.raw().hashee,
        };
        let substituted = return_type_g0f.into_raw().replace_debs(&substituter, 0);
        self.evaluator.eval(substituted)
    }
}
