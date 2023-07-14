use super::*;

impl TypeChecker {
    pub fn get_type_of_match(
        &mut self,
        match_: RcHashed<cst::Match>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<NormalForm, TypeError> {
        self.perform_match_precheck(match_.clone(), tcon, scon)?;

        let return_type_ast = self
            .cst_converter
            .convert(match_.hashee.return_type.clone());
        let normalized_return_type = self.evaluator.eval(return_type_ast);
        Ok(normalized_return_type)
    }

    fn perform_match_precheck(
        &mut self,
        match_: RcHashed<cst::Match>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let matchee_type = self.get_type(match_.hashee.matchee.clone(), tcon, scon)?;
        let Some((well_typed_matchee_type_ind, well_typed_matchee_type_args)) = matchee_type.clone().ind_or_ind_app() else {
            return Err(TypeError::NonInductiveMatcheeType {
                expr: match_.hashee.matchee.clone(),
                type_: matchee_type,
            });
        };

        let return_type_type = self.get_type(match_.hashee.return_type.clone(), tcon, scon)?;
        if !return_type_type.raw().is_universe() {
            return Err(TypeError::UnexpectedNonTypeExpression {
                expr: match_.hashee.return_type.clone(),
                type_: return_type_type,
            });
        }

        let vcon_count = well_typed_matchee_type_ind
            .raw()
            .hashee
            .vcon_defs
            .hashee
            .len();
        let match_case_count = match_.hashee.cases.len();
        if vcon_count != match_case_count {
            return Err(TypeError::WrongNumberOfMatchCases {
                match_: match_.hashee.clone(),
                matchee_type_ind: well_typed_matchee_type_ind.to_hashee().cloned(),
            });
        }

        let return_type_ast = self
            .cst_converter
            .convert(match_.hashee.return_type.clone());
        let unshifted_normalized_match_return_type = self.evaluator.eval(return_type_ast);
        self.perform_match_cases_precheck(
            match_,
            unshifted_normalized_match_return_type,
            well_typed_matchee_type_ind,
            well_typed_matchee_type_args,
            tcon,
            scon,
        )?;

        Ok(())
    }

    fn perform_match_cases_precheck(
        &mut self,
        match_: RcHashed<cst::Match>,
        unshifted_match_return_type: NormalForm,
        well_typed_matchee_type_ind: Normalized<RcSemHashed<ast::Ind>>,
        well_typed_matchee_type_args: Normalized<RcSemHashedVec<ast::Expr>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let vcon_defs: Normalized<&[ast::VconDef]> = well_typed_matchee_type_ind
            .to_hashee()
            .vcon_defs()
            .hashee()
            .derefed();

        for match_case_index in 0..match_.hashee.cases.len() {
            let well_typed_vcon_def = vcon_defs.index_ref(match_case_index);
            let match_case = &match_.hashee.cases[match_case_index];
            self.perform_match_case_precheck(
                match_case,
                match_case_index,
                well_typed_vcon_def,
                match_.clone(),
                unshifted_match_return_type.clone(),
                well_typed_matchee_type_ind.clone(),
                well_typed_matchee_type_args.clone(),
                tcon,
                scon,
            )?;
        }

        Ok(())
    }

    fn perform_match_case_precheck(
        &mut self,
        match_case: &cst::MatchCase,
        match_case_index: usize,
        well_typed_vcon_def: Normalized<&ast::VconDef>,
        match_: RcHashed<cst::Match>,
        unshifted_match_return_type: NormalForm,
        well_typed_matchee_type_ind: Normalized<RcSemHashed<ast::Ind>>,
        well_typed_matchee_type_args: Normalized<RcSemHashedVec<ast::Expr>>,
        tcon: LazyTypeContext,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let ind_singleton: [ast::Expr; 1] = [well_typed_matchee_type_ind.clone().into_raw().into()];
        let ind_singleton_deb_substituter = DebDownshiftSubstituter {
            new_exprs: &ind_singleton,
        };

        let actual_arity = match_case.arity.value;
        let expected_arity = well_typed_vcon_def.raw().param_types.hashee.len();
        if actual_arity != expected_arity {
            return Err(TypeError::WrongMatchCaseArity {
                actual_node: match_case.arity.clone(),
                actual: actual_arity,
                expected: expected_arity,
                match_: match_.hashee.clone(),
                match_case_index,
            });
        }

        let match_case_param_types = well_typed_vcon_def
            .raw()
            .param_types
            .clone()
            .replace_debs_with_increasing_cutoff(&ind_singleton_deb_substituter, 0);
        let match_case_param_types = self.evaluator.eval_expressions(match_case_param_types);
        let match_case_param_types = match_case_param_types.to_hashee();
        let tcon_with_match_case_param_types =
            LazyTypeContext::Snoc(&tcon, match_case_param_types.derefed());

        let match_case_param_count = match_case_param_types.raw().len();
        let substituted_vcon_index_args = well_typed_vcon_def
            .index_args()
            .replace_deb0_with_ind_with_increasing_cutoff(well_typed_matchee_type_ind.clone(), 0);
        let upshifted_matchee_type_args = well_typed_matchee_type_args
            .clone()
            .upshift_with_constant_cutoff(match_case_param_count);
        let extended_tcon_len = tcon_with_match_case_param_types.len();
        let matchee_ast = self.cst_converter.convert(match_.hashee.matchee.clone());
        let upshifted_matchee = matchee_ast.replace_debs(&DebUpshifter(match_case_param_count), 0);
        let upshifted_normalized_matchee = self.evaluator.eval(upshifted_matchee);
        let parameterized_vcon_capp = Normalized::vcon_capp(
            // TODO: Upshift `well_typed_matchee_type_ind`
            // by `match_case_param_count` WITH A CUTOFF OF ZERO.
            well_typed_matchee_type_ind,
            match_case_index,
            match_case_param_count,
        );
        let new_substitutions: Vec<LazySubstitution> =
            (0..substituted_vcon_index_args.raw().hashee.len())
                .map(|i| {
                    // TODO: Upshift `substituted_vcon_index_args`
                    // by `match_case_param_count` WITH A CUTOFF OF
                    // `match_case_param_count`.
                    let vcon_index_arg = substituted_vcon_index_args
                        .to_hashee()
                        .derefed()
                        .index_ref(i)
                        .cloned();
                    // NO ACTION NEEDED:
                    // We already upshifted the matchee type args.
                    // TODO: Delete above comment after we finish
                    // checking the shifting logic.
                    let matchee_index_arg = upshifted_matchee_type_args
                        .to_hashee()
                        .derefed()
                        .index_ref(i)
                        .cloned();
                    LazySubstitution {
                        tcon_len: extended_tcon_len,
                        from: vcon_index_arg,
                        to: matchee_index_arg,
                    }
                })
                .chain(std::iter::once(LazySubstitution {
                    tcon_len: extended_tcon_len,
                    // NO ACTION NEEDED:
                    // We already upshifted the normalized matchee.
                    // TODO: Delete above comment after we finish
                    // checking the shifting logic.
                    from: upshifted_normalized_matchee,
                    to: parameterized_vcon_capp,
                }))
                .collect();
        let extended_scon = LazySubstitutionContext::Snoc(&scon, &new_substitutions);

        let match_case_return_type = self.get_type(
            match_case.return_val.clone(),
            tcon_with_match_case_param_types,
            extended_scon,
        )?;

        let shifted_match_return_type = unshifted_match_return_type.upshift(expected_arity);
        // TODO: Replace with normal `?` syntax.
        let res = self.assert_expected_type_equality_holds_after_applying_scon(
            ExpectedTypeEquality {
                expr: match_case.return_val.clone(),
                expected_type: shifted_match_return_type,
                actual_type: match_case_return_type.clone(),
                tcon_len: extended_tcon_len,
            },
            extended_scon,
        );
        if let Err(err) = res {
            println!("****START_ERR****\n\n");
            println!(
                "****match_case_param_types.len:****\n{}\n\n",
                match_case_param_types.raw().len()
            );
            for raw_deb in 0..tcon.len() {
                println!(
                    "****tcon[{raw_deb}]:****\n{}\n\n",
                    PrettyPrinted(tcon.get(Deb(raw_deb)).unwrap().raw())
                );
                println!(
                    "****tcon.UNSHIFTED[{raw_deb}]:****\n{}\n\n",
                    PrettyPrinted(tcon.get_unshifted(Deb(raw_deb)).unwrap().raw())
                );
            }

            for raw_deb in 0..tcon_with_match_case_param_types.len() {
                println!(
                    "****tcon_with_match_case_param_types[{raw_deb}]:****\n{}\n\n",
                    PrettyPrinted(
                        tcon_with_match_case_param_types
                            .get(Deb(raw_deb))
                            .unwrap()
                            .raw()
                    )
                );
                println!(
                    "****tcon_with_match_case_param_types.UNSHIFTED[{raw_deb}]:****\n{}\n\n",
                    PrettyPrinted(
                        tcon_with_match_case_param_types
                            .get_unshifted(Deb(raw_deb))
                            .unwrap()
                            .raw()
                    )
                );
            }
            return Err(err);
        }

        Ok(())
    }
}
