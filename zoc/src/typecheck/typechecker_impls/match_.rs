use super::*;

impl TypeChecker {
    pub fn get_type_of_match(
        &mut self,
        match_g0: RcHashed<cst::Match>,
        tcon_g0: LazyTypeContext,
    ) -> Result<NormalForm, TypeError> {
        let matchee_type_g0 = self.get_type(match_g0.hashee.matchee.clone(), tcon_g0)?;

        let (matchee_type_ind_g0, matchee_type_args_g0) = self.assert_matchee_type_is_inductive(
            match_g0.hashee.matchee.clone(),
            matchee_type_g0.clone(),
        )?;

        self.assert_number_of_match_cases_is_correct(
            match_g0.clone(),
            matchee_type_ind_g0.clone(),
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
        self.assert_expr_type_is_universe(match_g0.hashee.return_type.clone(), tcon_g0matchparams)?;

        self.typecheck_match_cases_assuming_number_of_cases_is_correct(
            match_g0.clone(),
            matchee_type_ind_g0.clone(),
            matchee_type_args_g0.clone(),
            tcon_g0,
        )?;

        let matchee_g0 = self.cst_converter.convert(match_g0.hashee.matchee.clone());
        let substituter_new_exprs: Vec<ast::Expr> = matchee_type_args_g0
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
            .cst_converter
            .convert(match_g0.hashee.return_type.clone())
            .replace_debs(&substituter, 0);
        let normalized_return_type = self.evaluator.eval(return_type);
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
        matchee_type_ind: Normalized<RcHashed<ast::Ind>>,
        matchee_type_args: Normalized<RcHashedVec<ast::Expr>>,
        tcon: LazyTypeContext,
    ) -> Result<(), TypeError> {
        for i in 0..match_.hashee.cases.len() {
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

    fn typecheck_match_case(
        &mut self,
        case_index: usize,
        match_g0: RcHashed<cst::Match>,
        matchee_type_ind_g0: Normalized<RcHashed<ast::Ind>>,
        matchee_type_args_g0: Normalized<RcHashedVec<ast::Expr>>,
        tcon_g0: LazyTypeContext,
    ) -> Result<(), TypeError> {
        let case = &match_g0.hashee.cases[case_index];
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
        let case_arity = case.arity.value;

        let extended_tcon_g1 =
            LazyTypeContext::Snoc(&tcon_g0, param_types_g0.to_hashee().derefed());

        let case_return_val_type_g1 = self.get_type(case.return_val.clone(), extended_tcon_g1)?;

        // TODO: Clean this up.
        let match_arity = 1 + matchee_type_args_g0.raw().hashee.len();
        let vcon_type_cfor_return_type_capp_args_g1 = vcon_type_g0
            .for_return_type_or_self()
            .app_args_or_empty_vec();
        let matchee_type_ind_g1 = matchee_type_ind_g0.upshift(case_arity, 0);
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
            .cst_converter
            .convert(match_g0.hashee.return_type.clone());
        let match_return_type_g1 = match_return_type_g0matchparams
            .replace_debs(&DebUpshifter(case_arity), match_arity)
            .replace_debs(&substituter, 0);
        let normalized_match_return_type_g1 = self.evaluator.eval(match_return_type_g1);

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
}
