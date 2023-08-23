use super::*;

impl TypeChecker {
    pub fn get_type_of_match<A: AuxDataFamily>(
        &mut self,
        match_g0: RcHashed<ast::Match<A>>,
        tcon_g0: LazyTypeContext,
    ) -> Result<NormalForm, TypeError<A>> {
        let matchee_type_g0 = self.get_type(match_g0.hashee.matchee.clone(), tcon_g0)?;

        let (matchee_type_ind_g0, matchee_type_args_g0) = self.assert_matchee_type_is_inductive(
            match_g0.hashee.matchee.clone(),
            matchee_type_g0.clone(),
        )?;

        self.assert_number_of_match_cases_is_correct(
            match_g0.clone(),
            matchee_type_ind_g0.clone(),
        )?;

        self.assert_stated_return_type_arity_is_correct(
            match_g0.clone(),
            matchee_type_args_g0.clone(),
        )?;

        let tcon_extension = {
            let matchee_type_ind_index_types_g0 = matchee_type_ind_g0.to_hashee().index_types();
            let mut out = matchee_type_ind_index_types_g0.hashee().cloned();
            let ind_capp_g0matchparamspartial =
                NormalForm::ind_capp_of_descending_debs(matchee_type_ind_g0.clone());
            out.push(ind_capp_g0matchparamspartial);
            out
        };
        let tcon_g0matchparams = LazyTypeContext::Snoc(&tcon_g0, tcon_extension.to_derefed());
        let return_type_type = self.assert_expr_type_is_universe(
            match_g0.hashee.return_type.clone(),
            tcon_g0matchparams,
        )?;

        self.typecheck_match_cases_assuming_number_of_cases_is_correct(
            match_g0.clone(),
            matchee_type_ind_g0.clone(),
            matchee_type_args_g0.clone(),
            tcon_g0,
        )?;

        let matchee_g0 = self.aux_remover.convert(match_g0.hashee.matchee.clone());
        let substituter_new_exprs: Vec<minimal_ast::Expr> = matchee_type_args_g0
            .raw()
            .hashee
            .iter()
            .cloned()
            .chain(std::iter::once(matchee_g0))
            .collect();
        let substituter = DebDownshiftSubstituter {
            new_exprs: &substituter_new_exprs,
        };
        let return_type = self
            .aux_remover
            .convert(match_g0.hashee.return_type.clone())
            .replace_debs(&substituter, 0);
        let normalized_return_type = self.evaluator.eval(return_type);

        self.check_erasability(match_g0, matchee_type_ind_g0, return_type_type, tcon_g0)?;

        Ok(normalized_return_type)
    }

    fn assert_matchee_type_is_inductive<A: AuxDataFamily>(
        &mut self,
        matchee: ast::Expr<A>,
        matchee_type: NormalForm,
    ) -> Result<
        (
            Normalized<RcHashed<minimal_ast::Ind>>,
            Normalized<RcHashedVec<minimal_ast::Expr>>,
        ),
        TypeError<A>,
    > {
        if let Some(ind_and_args) = matchee_type.clone().ind_or_ind_app() {
            return Ok(ind_and_args);
        }

        Err(TypeError::NonInductiveMatcheeType {
            expr: matchee,
            type_: matchee_type,
        })
    }

    fn assert_number_of_match_cases_is_correct<A: AuxDataFamily>(
        &mut self,
        match_: RcHashed<ast::Match<A>>,
        matchee_type_ind: Normalized<RcHashed<minimal_ast::Ind>>,
    ) -> Result<(), TypeError<A>> {
        let expected = matchee_type_ind.raw().hashee.vcon_defs.hashee.len();
        let actual = match_.hashee.cases.hashee.len();
        if expected != actual {
            return Err(TypeError::WrongNumberOfMatchCases {
                match_: match_.hashee.clone(),
                matchee_type_ind: matchee_type_ind.to_hashee().cloned(),
            });
        }

        Ok(())
    }

    fn assert_stated_return_type_arity_is_correct<A: AuxDataFamily>(
        &mut self,
        match_: RcHashed<ast::Match<A>>,
        matchee_type_args: Normalized<RcHashedVec<minimal_ast::Expr>>,
    ) -> Result<(), TypeError<A>> {
        let correct_return_type_arity = 1 + matchee_type_args.raw().hashee.len();

        if match_.hashee.return_type_arity != correct_return_type_arity {
            let matchee_type_args = matchee_type_args.to_hashee().into_vec();
            return Err(TypeError::WrongMatchReturnTypeArity {
                match_: match_.hashee.clone(),
                matchee_type_args,
            });
        }

        Ok(())
    }

    fn typecheck_match_cases_assuming_number_of_cases_is_correct<A: AuxDataFamily>(
        &mut self,
        match_: RcHashed<ast::Match<A>>,
        matchee_type_ind: Normalized<RcHashed<minimal_ast::Ind>>,
        matchee_type_args: Normalized<RcHashedVec<minimal_ast::Expr>>,
        tcon: LazyTypeContext,
    ) -> Result<(), TypeError<A>> {
        for i in 0..match_.hashee.cases.hashee.len() {
            self.typecheck_match_case(
                i,
                match_.clone(),
                matchee_type_ind.clone(),
                matchee_type_args.clone(),
                tcon,
            )?;
        }
        Ok(())
    }

    fn typecheck_match_case<A: AuxDataFamily>(
        &mut self,
        case_index: usize,
        match_g0: RcHashed<ast::Match<A>>,
        matchee_type_ind_g0: Normalized<RcHashed<minimal_ast::Ind>>,
        matchee_type_args_g0: Normalized<RcHashedVec<minimal_ast::Expr>>,
        tcon_g0: LazyTypeContext,
    ) -> Result<(), TypeError<A>> {
        let case = &match_g0.hashee.cases.hashee[case_index];
        let vcon_type_g0 = self.get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
            matchee_type_ind_g0.clone(),
            case_index,
        );

        let param_types_g0 = vcon_type_g0.clone().for_param_types_or_empty_vec();

        self.assert_stated_case_arity_is_correct(
            case.arity,
            param_types_g0.raw().hashee.len(),
            case_index,
            match_g0.clone(),
        )?;

        let extended_tcon_g1 =
            LazyTypeContext::Snoc(&tcon_g0, param_types_g0.to_hashee().derefed());

        let case_return_val_type_g1 = self.get_type(case.return_val.clone(), extended_tcon_g1)?;

        // TODO: Clean this up.
        let match_arity = 1 + matchee_type_args_g0.raw().hashee.len();
        let vcon_type_cfor_return_type_capp_args_g1 = vcon_type_g0
            .for_return_type_or_self()
            .app_args_or_empty_vec();
        let matchee_type_ind_g1 = matchee_type_ind_g0.upshift(case.arity, 0);
        let vcon_capp_g1 =
            NormalForm::vcon_capp_of_descending_debs(matchee_type_ind_g1, case_index).into_raw();
        let substituter_new_exprs = vcon_type_cfor_return_type_capp_args_g1
            .raw()
            .hashee
            .iter()
            .cloned()
            .chain(std::iter::once(vcon_capp_g1))
            .collect::<Vec<_>>();
        let substituter = DebDownshiftSubstituter {
            new_exprs: &substituter_new_exprs,
        };
        let match_return_type_g0matchparams = self
            .aux_remover
            .convert(match_g0.hashee.return_type.clone());
        let match_return_type_g1 = match_return_type_g0matchparams
            .replace_debs(&DebUpshifter(case.arity), match_arity)
            .replace_debs(&substituter, 0);
        let normalized_match_return_type_g1 = self.evaluator.eval(match_return_type_g1);

        self.assert_expected_type_equality_holds(ExpectedTypeEquality {
            expr: case.return_val.clone(),
            expected_type: normalized_match_return_type_g1,
            actual_type: case_return_val_type_g1,
        })?;

        Ok(())
    }

    fn assert_stated_case_arity_is_correct<A: AuxDataFamily>(
        &mut self,
        stated_arity: usize,
        expected_arity: usize,
        match_case_index: usize,
        match_: RcHashed<ast::Match<A>>,
    ) -> Result<(), TypeError<A>> {
        if stated_arity != expected_arity {
            return Err(TypeError::WrongMatchCaseArity {
                stated_arity,
                expected: expected_arity,
                match_: match_.hashee.clone(),
                match_case_index,
            });
        }

        Ok(())
    }
}

impl TypeChecker {
    fn check_erasability<A: AuxDataFamily>(
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
            .aux_remover
            .convert_expressions(&ind_g0.index_types.hashee);
        let normalized_index_types_g0 = self.evaluator.eval_expressions(index_types_g0_minimal);
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
            .get_types_of_dependent_expressions(
                &vcon_def_g1.param_types.hashee,
                tcon_with_ind_type_g1,
            )
            .map_err(|err| err.remove_ast_aux_data(&mut self.aux_remover))
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

// TODO: Consider whether we should add an exception
// to the erasability rules for when
// the matchee has a vcon at the top.
