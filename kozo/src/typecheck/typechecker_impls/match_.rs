use super::*;

// TODO: Remove unused params.

impl TypeChecker {
    pub fn get_type_of_match(
        &mut self,
        match_: RcHashed<cst::Match>,
        tcon: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        let matchee_type = self.get_type(match_.hashee.matchee.clone(), tcon)?;

        let matchee_ast = self.cst_converter.convert(match_.hashee.matchee.clone());
        let normalized_matchee = self.evaluator.eval(matchee_ast);

        let (matchee_type_ind, matchee_type_args) = self.assert_matchee_type_is_inductive(
            match_.hashee.matchee.clone(),
            matchee_type.clone(),
        )?;

        self.assert_number_of_match_cases_is_correct(match_.clone(), matchee_type_ind.clone())?;

        let normalized_return_type = self
            .assert_expr_type_is_universe_and_then_eval(match_.hashee.return_type.clone(), tcon)?;

        self.typecheck_match_cases_assuming_number_of_cases_is_correct(
            match_,
            normalized_matchee,
            matchee_type_ind,
            matchee_type_args,
            normalized_return_type.clone(),
            tcon,
        )?;

        Ok(normalized_return_type)
    }

    fn assert_matchee_type_is_inductive(
        &mut self,
        matchee: cst::Expr,
        matchee_type: NormalForm,
    ) -> Result<
        (
            Normalized<RcHashed<ast::Ind>>,
            Normalized<RcHashedVec<ast::Expr>>,
        ),
        TypeError,
    > {
        if let Some(ind_and_args) = matchee_type.clone().ind_or_ind_app() {
            return Ok(ind_and_args);
        }

        Err(TypeError::NonInductiveMatcheeType {
            expr: matchee,
            type_: matchee_type,
        })
    }

    fn assert_number_of_match_cases_is_correct(
        &mut self,
        match_: RcHashed<cst::Match>,
        matchee_type_ind: Normalized<RcHashed<ast::Ind>>,
    ) -> Result<(), TypeError> {
        let expected = matchee_type_ind.raw().hashee.vcon_defs.hashee.len();
        let actual = match_.hashee.cases.len();
        if expected != actual {
            return Err(TypeError::WrongNumberOfMatchCases {
                match_: match_.hashee.clone(),
                matchee_type_ind: matchee_type_ind.to_hashee().cloned(),
            });
        }

        Ok(())
    }

    fn typecheck_match_cases_assuming_number_of_cases_is_correct(
        &mut self,
        match_: RcHashed<cst::Match>,
        normalized_matchee: NormalForm,
        matchee_type_ind: Normalized<RcHashed<ast::Ind>>,
        matchee_type_args: Normalized<RcHashedVec<ast::Expr>>,
        normalized_match_return_type: NormalForm,
        tcon: LazyTypeContext,
    ) -> Result<(), TypeError> {
        for i in 0..match_.hashee.cases.len() {
            self.typecheck_match_case(
                i,
                match_.clone(),
                normalized_matchee.clone(),
                matchee_type_ind.clone(),
                matchee_type_args.clone(),
                normalized_match_return_type.clone(),
                tcon,
            )?;
        }
        Ok(())
    }

    fn typecheck_match_case(
        &mut self,
        case_index: usize,
        match_g0: RcHashed<cst::Match>,
        normalized_matchee_g0: NormalForm,
        matchee_type_ind_g0: Normalized<RcHashed<ast::Ind>>,
        matchee_type_args_g0: Normalized<RcHashedVec<ast::Expr>>,
        normalized_match_return_type_g0: NormalForm,
        tcon_g0: LazyTypeContext,
    ) -> Result<(), TypeError> {
        let case = &match_g0.hashee.cases[case_index];
        match case {
            cst::MatchCase::Nondismissed(case) => self.typecheck_nondismissed_match_case(
                case_index,
                case,
                match_g0.clone(),
                normalized_matchee_g0,
                matchee_type_ind_g0,
                matchee_type_args_g0,
                normalized_match_return_type_g0,
                tcon_g0,
            ),

            cst::MatchCase::Dismissed(_) => self.typecheck_dismissed_match_case(
                case_index,
                match_g0.clone(),
                normalized_matchee_g0,
                matchee_type_ind_g0,
                matchee_type_args_g0,
                tcon_g0,
            ),
        }
    }

    fn typecheck_nondismissed_match_case(
        &mut self,
        case_index: usize,
        case: &cst::NondismissedMatchCase,
        match_g0: RcHashed<cst::Match>,
        _normalized_matchee_g0: NormalForm,
        matchee_type_ind_g0: Normalized<RcHashed<ast::Ind>>,
        _matchee_type_args_g0: Normalized<RcHashedVec<ast::Expr>>,
        normalized_match_return_type_g0: NormalForm,
        tcon_g0: LazyTypeContext,
    ) -> Result<(), TypeError> {
        let vcon_type_g0 = self.get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
            matchee_type_ind_g0.clone(),
            case_index,
        );

        let param_types_g0 = vcon_type_g0.clone().for_param_types_or_empty_vec();
        let param_count = param_types_g0.raw().hashee.len();

        self.assert_stated_case_arity_is_correct(
            case.arity,
            param_count,
            case_index,
            match_g0.clone(),
        )?;

        let extended_tcon_g1 =
            LazyTypeContext::Snoc(&tcon_g0, param_types_g0.to_hashee().derefed());

        let case_return_val_type_g1 = self.get_type(case.return_val.clone(), extended_tcon_g1)?;

        let normalized_match_return_type_g1 =
            normalized_match_return_type_g0.upshift(param_count, 0);

        self.assert_expected_type_equality_holds_after_applying_scon(ExpectedTypeEquality {
            expr: case.return_val.clone(),
            expected_type: normalized_match_return_type_g1,
            actual_type: case_return_val_type_g1,
        })?;

        Ok(())
    }

    fn assert_stated_case_arity_is_correct(
        &mut self,
        stated_arity: cst::NumberLiteral,
        expected_arity: usize,
        match_case_index: usize,
        match_: RcHashed<cst::Match>,
    ) -> Result<(), TypeError> {
        if stated_arity.value != expected_arity {
            return Err(TypeError::WrongMatchCaseArity {
                actual_node: stated_arity,
                expected: expected_arity,
                match_: match_.hashee.clone(),
                match_case_index,
            });
        }

        Ok(())
    }

    // TODO: Delete this once we remove `contra` syntax.
    fn typecheck_dismissed_match_case(
        &mut self,
        case_index: usize,
        match_g0: RcHashed<cst::Match>,
        _normalized_matchee_g0: NormalForm,
        _matchee_type_ind_g0: Normalized<RcHashed<ast::Ind>>,
        _matchee_type_args_g0: Normalized<RcHashedVec<ast::Expr>>,
        _tcon_g0: LazyTypeContext,
    ) -> Result<(), TypeError> {
        return Err(TypeError::IllegallyDismissedMatchCase {
            match_: match_g0.hashee.clone(),
            match_case_index: case_index,
        });
    }
}
