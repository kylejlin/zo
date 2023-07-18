use crate::pretty_print::PrettyPrinted;

use super::*;

impl TypeChecker {
    pub fn get_type_of_match(
        &mut self,
        match_: RcHashed<cst::Match>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        let matchee_type = self.get_type(match_.hashee.matchee.clone(), tcon, scon)?;

        let matchee_ast = self.cst_converter.convert(match_.hashee.matchee.clone());
        let normalized_matchee = self.evaluator.eval(matchee_ast);

        let (matchee_type_ind, matchee_type_args) = self.assert_matchee_type_is_inductive(
            match_.hashee.matchee.clone(),
            matchee_type.clone(),
            scon,
            tcon.len(),
        )?;

        self.assert_number_of_match_cases_is_correct(match_.clone(), matchee_type_ind.clone())?;

        let normalized_return_type = self.assert_expr_type_is_universe_and_then_eval(
            match_.hashee.return_type.clone(),
            tcon,
            scon,
        )?;

        self.typecheck_match_cases_assuming_number_of_cases_is_correct(
            match_,
            normalized_matchee,
            matchee_type_ind,
            matchee_type_args,
            normalized_return_type.clone(),
            tcon,
            scon,
        )?;

        Ok(normalized_return_type)
    }

    fn assert_matchee_type_is_inductive(
        &mut self,
        matchee: cst::Expr,
        matchee_type: NormalForm,
        scon: LazySubstitutionContext,
        tcon_len: usize,
    ) -> Result<
        (
            Normalized<RcSemHashed<ast::Ind>>,
            Normalized<RcSemHashedVec<ast::Expr>>,
        ),
        TypeError,
    > {
        if let Some(ind_and_args) = matchee_type.clone().ind_or_ind_app() {
            return Ok(ind_and_args);
        }

        let subs = scon.into_concrete_noncompounded_substitutions(tcon_len);
        let substituted_matchee_type = self
            .apply_concrete_substitutions(subs, [matchee_type.clone()])
            .0[0]
            .clone();
        if let Some(ind_and_args) = substituted_matchee_type.clone().ind_or_ind_app() {
            return Ok(ind_and_args);
        }

        Err(TypeError::NonInductiveMatcheeType {
            expr: matchee,
            type_: matchee_type,
            type_after_applying_scon: substituted_matchee_type,
        })
    }

    fn assert_number_of_match_cases_is_correct(
        &mut self,
        match_: RcHashed<cst::Match>,
        matchee_type_ind: Normalized<RcSemHashed<ast::Ind>>,
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
        matchee_type_ind: Normalized<RcSemHashed<ast::Ind>>,
        matchee_type_args: Normalized<RcSemHashedVec<ast::Expr>>,
        normalized_match_return_type: NormalForm,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
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
                scon,
            )?;
        }
        Ok(())
    }

    fn typecheck_match_case(
        &mut self,
        case_index: usize,
        match_g0: RcHashed<cst::Match>,
        normalized_matchee_g0: NormalForm,
        matchee_type_ind_g0: Normalized<RcSemHashed<ast::Ind>>,
        matchee_type_args_g0: Normalized<RcSemHashedVec<ast::Expr>>,
        normalized_match_return_type_g0: NormalForm,
        tcon_g0: LazyTypeContext,
        scon: LazySubstitutionContext,
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
                scon,
            ),

            cst::MatchCase::Dismissed(_) => self.typecheck_dismissed_match_case(
                case_index,
                match_g0.clone(),
                normalized_matchee_g0,
                matchee_type_ind_g0,
                matchee_type_args_g0,
                tcon_g0,
                scon,
            ),
        }
    }

    fn typecheck_nondismissed_match_case(
        &mut self,
        case_index: usize,
        case: &cst::NondismissedMatchCase,
        match_g0: RcHashed<cst::Match>,
        normalized_matchee_g0: NormalForm,
        matchee_type_ind_g0: Normalized<RcSemHashed<ast::Ind>>,
        matchee_type_args_g0: Normalized<RcSemHashedVec<ast::Expr>>,
        normalized_match_return_type_g0: NormalForm,
        tcon_g0: LazyTypeContext,
        scon: LazySubstitutionContext,
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

        let vcon_type_g1 = vcon_type_g0.upshift(param_count, 0);
        let new_substitutions = Self::get_new_substitutions(
            case_index,
            normalized_matchee_g0.upshift(param_count, 0),
            matchee_type_ind_g0.upshift(param_count, 0),
            matchee_type_args_g0.upshift_with_constant_cutoff(param_count),
            vcon_type_g1,
            extended_tcon_g1.len(),
        );
        let extended_scon = LazySubstitutionContext::Snoc(&scon, &new_substitutions);

        let case_return_val_type_g1 =
            self.get_type(case.return_val.clone(), extended_tcon_g1, extended_scon)?;

        let normalized_match_return_type_g1 =
            normalized_match_return_type_g0.upshift(param_count, 0);

        // TODO: Restore to `?`.
        let res = self.assert_expected_type_equality_holds_after_applying_scon(
            ExpectedTypeEquality {
                expr: case.return_val.clone(),
                expected_type: normalized_match_return_type_g1,
                actual_type: case_return_val_type_g1,
                tcon_len: extended_tcon_g1.len(),
            },
            extended_scon,
        );
        if res.is_err() {
            let subs =
                extended_scon.into_concrete_noncompounded_substitutions(extended_tcon_g1.len());
            for (i, sub) in subs.iter().enumerate() {
                println!("sub[{}].from = {}", i, sub.from().raw().pretty_printed());
                println!("sub[{}].to = {}", i, sub.to().raw().pretty_printed());
            }
        }
        res?;

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

    fn get_new_substitutions(
        case_index: usize,
        normalized_matchee_g1: NormalForm,
        matchee_type_ind_g1: Normalized<RcSemHashed<ast::Ind>>,
        matchee_type_args_g1: Normalized<RcSemHashedVec<ast::Expr>>,
        vcon_type_g1: NormalForm,
        tcon_g1_len: usize,
    ) -> Vec<LazySubstitution> {
        let vcon_param_count = vcon_type_g1
            .clone()
            .for_param_types_or_empty_vec()
            .raw()
            .hashee
            .len();

        let vcon_index_args_g1 = vcon_type_g1
            .for_return_type_or_self()
            .app_args_or_empty_vec()
            .downshift_n_with_constant_cutoff_n(vcon_param_count);

        let index_substitutions = (0..vcon_index_args_g1.raw().hashee.len()).map(|i| {
            let vcon_index_arg_g1 = vcon_index_args_g1.to_hashee().index(i).cloned();
            let matchee_type_arg_g1 = matchee_type_args_g1.to_hashee().index(i).cloned();
            LazySubstitution {
                tcon_len: tcon_g1_len,
                tentative_from: vcon_index_arg_g1,
                tentative_to: matchee_type_arg_g1,
            }
        });

        let capp_g1 = NormalForm::vcon_capp(matchee_type_ind_g1, case_index);

        let matchee_substitution = LazySubstitution {
            tcon_len: tcon_g1_len,
            tentative_from: normalized_matchee_g1,
            tentative_to: capp_g1,
        };

        let substitutions: Vec<LazySubstitution> = index_substitutions
            .chain(std::iter::once(matchee_substitution))
            .collect();

        substitutions
    }

    fn typecheck_dismissed_match_case(
        &mut self,
        case_index: usize,
        match_g0: RcHashed<cst::Match>,
        normalized_matchee_g0: NormalForm,
        matchee_type_ind_g0: Normalized<RcSemHashed<ast::Ind>>,
        matchee_type_args_g0: Normalized<RcSemHashedVec<ast::Expr>>,
        tcon_g0: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let vcon_type_g0 = self.get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
            matchee_type_ind_g0.clone(),
            case_index,
        );

        let param_types_g0 = vcon_type_g0.clone().for_param_types_or_empty_vec();
        let param_count = param_types_g0.raw().hashee.len();

        let extended_tcon_g1 =
            LazyTypeContext::Snoc(&tcon_g0, param_types_g0.to_hashee().derefed());

        let vcon_type_g1 = vcon_type_g0.upshift(param_count, 0);
        let new_substitutions = Self::get_new_substitutions(
            case_index,
            normalized_matchee_g0.upshift(param_count, 0),
            matchee_type_ind_g0.upshift(param_count, 0),
            matchee_type_args_g0.upshift_with_constant_cutoff(param_count),
            vcon_type_g1,
            extended_tcon_g1.len(),
        );
        let extended_scon = LazySubstitutionContext::Snoc(&scon, &new_substitutions);

        let subs = extended_scon.into_concrete_noncompounded_substitutions(extended_tcon_g1.len());
        let ([], HasExploded(has_exploded)) = self.apply_concrete_substitutions(subs, []);

        if !has_exploded {
            return Err(TypeError::IllegallyDismissedMatchCase {
                match_: match_g0.hashee.clone(),
                match_case_index: case_index,
            });
        }

        Ok(())
    }
}
