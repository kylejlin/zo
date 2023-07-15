use super::*;

impl TypeChecker {
    pub fn get_type_of_app(
        &mut self,
        app: RcHashed<cst::App>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let callee_type = self.get_type(app.hashee.callee.clone(), tcon, scon)?;
        let callee_type = self.assert_callee_type_is_a_for_expression(
            callee_type,
            app.clone(),
            scon,
            tcon.len(),
        )?;

        let callee_type_param_types = callee_type.to_hashee().param_types().cloned();
        let callee_type_return_type_g0f = callee_type.to_hashee().return_type().cloned();

        self.assert_arg_count_is_correct(app.clone(), callee_type.clone())?;

        let arg_types = self.get_types_of_independent_expressions(&app.hashee.args, tcon, scon)?;

        let args_ast = self
            .cst_converter
            .convert_expressions(app.hashee.args.clone());
        let normalized_args = self.evaluator.eval_expressions(args_ast);

        let substituted_callee_type_param_types = self.substitute_callee_type_param_types(
            callee_type_param_types.clone(),
            normalized_args.clone(),
        );

        self.assert_expected_type_equalities_holds_after_applying_scon(
            ExpectedTypeEqualities {
                exprs: app.hashee.args.to_vec_of_cloned(),
                expected_types: substituted_callee_type_param_types.to_hashee().cloned(),
                actual_types: arg_types,
                tcon_len: tcon.len(),
            },
            scon,
        )?;

        let substituted_callee_type_return_type =
            self.substitute_callee_type_return_type(callee_type_return_type_g0f, normalized_args);
        Ok(substituted_callee_type_return_type)
    }

    fn assert_callee_type_is_a_for_expression(
        &mut self,
        callee_type: NormalForm,
        app: RcHashed<cst::App>,
        scon: LazySubstitutionContext,
        tcon_len: usize,
    ) -> Result<Normalized<RcSemHashed<ast::For>>, TypeError> {
        if let Ok(for_) = callee_type.clone().try_into_for() {
            return Ok(for_);
        }

        let subs = scon.into_concrete_noncompounded_substitutions(tcon_len);
        let callee_type_after_applying_scon =
            self.apply_concrete_substitutions(subs, [callee_type.clone()])[0].clone();
        if let Ok(for_) = callee_type_after_applying_scon.clone().try_into_for() {
            return Ok(for_);
        }

        Err(TypeError::CalleeTypeIsNotAForExpression {
            app: app.hashee.clone(),
            callee_type,
            callee_type_after_applying_scon,
        })
    }

    fn assert_arg_count_is_correct(
        &mut self,
        app: RcHashed<cst::App>,
        callee_type: Normalized<RcSemHashed<ast::For>>,
    ) -> Result<(), TypeError> {
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

        Ok(())
    }

    fn substitute_callee_type_param_types(
        &mut self,
        param_types: Normalized<RcSemHashedVec<ast::Expr>>,
        args: Normalized<RcSemHashedVec<ast::Expr>>,
    ) -> Normalized<RcSemHashedVec<ast::Expr>> {
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
            .into_rc_sem_hashed()
    }

    fn substitute_callee_type_return_type(
        &mut self,
        return_type_g0f: NormalForm,
        args: Normalized<RcSemHashedVec<ast::Expr>>,
    ) -> NormalForm {
        let substituter = DebDownshiftSubstituter {
            new_exprs: &args.raw().hashee,
        };
        let substituted = return_type_g0f.into_raw().replace_debs(&substituter, 0);
        self.evaluator.eval(substituted)
    }
}
