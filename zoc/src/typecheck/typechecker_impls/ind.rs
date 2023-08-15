use super::*;

impl TypeChecker {
    pub fn get_type_of_ind(
        &mut self,
        ind: RcHashed<cst::Ind>,
        tcon_g0: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        // TODO: Check for strict positivity.
        // We can and should check this before
        // we perform the rest of the checks,
        // in order to prevent the chance of infinite recursion.
        //
        // Page 7 of https://inria.hal.science/hal-01094195/document
        // may possibly be of some use.

        let normalized_index_types_g0 = self
            .typecheck_and_normalize_param_types_with_limit(
                &ind.hashee.index_types,
                LimitToIndUniverse(ind.clone()),
                tcon_g0,
            )?
            .into_rc_hashed();

        let universe_node = NormalForm::universe(ast::UniverseNode {
            universe: Universe {
                level: UniverseLevel(ind.hashee.type_.level),
                erasable: ind.hashee.type_.erasable,
            },
        });
        let ind_type_g0 = Normalized::for_(normalized_index_types_g0.clone(), universe_node)
            .collapse_if_nullary();

        let ind_type_singleton = Normalized::<[_; 1]>::new(ind_type_g0.clone());
        let tcon_with_ind_type_g1 =
            LazyTypeContext::Snoc(&tcon_g0, ind_type_singleton.as_ref().convert_ref());

        self.typecheck_ind_vcon_defs(
            ind.clone(),
            normalized_index_types_g0,
            tcon_with_ind_type_g1,
        )?;

        self.get_positivity_checker()
            .check_ind_positivity_assuming_it_is_otherwise_well_typed(ind.clone())?;

        Ok(ind_type_g0)
    }

    fn typecheck_ind_vcon_defs(
        &mut self,
        ind: RcHashed<cst::Ind>,
        normalized_index_types_g0: Normalized<RcHashedVec<ast::Expr>>,
        tcon_g1: LazyTypeContext,
    ) -> Result<(), TypeError> {
        for def in &ind.hashee.vcon_defs {
            self.typecheck_ind_vcon_def(
                def,
                ind.clone(),
                normalized_index_types_g0.clone(),
                tcon_g1,
            )?;
        }
        Ok(())
    }

    fn typecheck_ind_vcon_def(
        &mut self,
        def: &cst::VconDef,
        ind: RcHashed<cst::Ind>,
        normalized_index_types_g0: Normalized<RcHashedVec<ast::Expr>>,
        tcon_g1: LazyTypeContext,
    ) -> Result<(), TypeError> {
        self.assert_index_arg_count_is_correct(def, normalized_index_types_g0.raw().hashee.len())?;

        let normalized_param_types_g1 = self.typecheck_and_normalize_param_types_with_limit(
            &def.param_types,
            LimitToIndUniverse(ind),
            tcon_g1,
        )?;

        let tcon_with_param_types_g2 =
            LazyTypeContext::Snoc(&tcon_g1, normalized_param_types_g1.to_derefed());

        let index_arg_types_g2 =
            self.get_types_of_independent_expressions(&def.index_args, tcon_with_param_types_g2)?;

        let index_args_ast = self.cst_converter.convert_expressions(&def.index_args);
        let normalized_index_args_g2 = self.evaluator.eval_expressions(index_args_ast);

        let normalized_index_types_g2 =
            normalized_index_types_g0.upshift_with_increasing_cutoff(1 + def.param_types.len());
        let normalized_index_types_g2 = self.substitute_callee_type_param_types(
            normalized_index_types_g2,
            normalized_index_args_g2,
        );

        self.assert_expected_type_equalities_holds(ExpectedTypeEqualities {
            exprs: &def.index_args,
            expected_types: normalized_index_types_g2.to_hashee().derefed(),
            actual_types: index_arg_types_g2.to_derefed(),
        })?;

        Ok(())
    }

    fn assert_index_arg_count_is_correct(
        &mut self,
        def: &cst::VconDef,
        expected_index_arg_count: usize,
    ) -> Result<(), TypeError> {
        let actual_index_arg_count = def.index_args.len();
        if expected_index_arg_count != actual_index_arg_count {
            return Err(TypeError::WrongNumberOfIndexArguments {
                def: def.clone(),
                expected: expected_index_arg_count,
                actual: actual_index_arg_count,
            });
        }

        Ok(())
    }

    fn get_positivity_checker(&mut self) -> PositivityChecker {
        PositivityChecker { typechecker: self }
    }
}
