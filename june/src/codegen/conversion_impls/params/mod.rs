use super::*;

impl JuneConverter {
    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_optional_typed_nonfun_param_defs_to_context_extension<'a>(
        &mut self,
        params: Option<&'a jnode::CommaSeparatedNonfunParamDefs>,
        context: Context,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>), SemanticError> {
        let Some(params) = params else {
            return Ok((vec![], vec![]));
        };

        self.convert_typed_nonfun_param_defs_to_context_extension(params, context)
    }

    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_typed_nonfun_param_defs_to_context_extension<'a>(
        &mut self,
        params: &'a jnode::CommaSeparatedNonfunParamDefs,
        context: Context,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>), SemanticError> {
        match params {
            jnode::CommaSeparatedNonfunParamDefs::One(param) => {
                let param_type = self.convert(&param.type_, context)?;
                let entry = self.get_deb_defining_entry(param.name.val());

                Ok((vec![entry], vec![param_type]))
            }

            jnode::CommaSeparatedNonfunParamDefs::Snoc(rdc, rac) => {
                let (mut entries, mut param_types) =
                    self.convert_typed_nonfun_param_defs_to_context_extension(rdc, context)?;

                let extended_context = Context::Snoc(&context, &entries);
                let rac_type = self.convert(&rac.type_, extended_context)?;

                entries.push(self.get_deb_defining_entry(rac.name.val()));
                param_types.push(rac_type);
                Ok((entries, param_types))
            }
        }
    }

    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index))`.
    pub(crate) fn convert_typed_fun_param_defs_to_context_extension<'a>(
        &mut self,
        params: &'a jnode::CommaSeparatedFunParamDefs,
        context: Context,
    ) -> Result<(Vec<UnshiftedEntry<'a>>, Vec<znode::Expr>, Option<usize>), SemanticError> {
        let (context_extension, param_types, decreasing_index_or_param_count) =
            self.convert_typed_fun_param_defs_to_context_extension_aux(params, context)?;

        let decreasing_index = match decreasing_index_or_param_count {
            DecIndexOrParamCount::DecIndex(i, _) => Some(i),
            DecIndexOrParamCount::ParamCount(_) => None,
        };

        Ok((context_extension, param_types, decreasing_index))
    }

    /// If the params are valid,
    /// this function returns `Ok((entries, param_types, dash_index_or_param_count))`.
    /// The difference between this function and `convert_typed_fun_param_defs_to_context_extension`
    /// is that this function returns `dash_index_or_param_count`, which is of type `DecIndexOrParamCount`.
    /// On the other hand, `convert_typed_fun_param_defs_to_context_extension` returns `decreasing_index`, which is of type `Option<usize>`.
    /// For this function:
    /// - If there is exactly one decreasing index `i`, then the value of `dash_index_or_param_count` is `DecIndex(i)`.
    /// - If there are no decreasing indices, then the value of `dash_index_or_param_count` is `ParamCount(n)`,
    ///   where `n` is the length of `params`.
    fn convert_typed_fun_param_defs_to_context_extension_aux<'a>(
        &mut self,
        params: &'a jnode::CommaSeparatedFunParamDefs,
        context: Context,
    ) -> Result<
        (
            Vec<UnshiftedEntry<'a>>,
            Vec<znode::Expr>,
            DecIndexOrParamCount<'a>,
        ),
        SemanticError,
    > {
        match params {
            jnode::CommaSeparatedFunParamDefs::One(param) => {
                let param_type = self.convert(&param.type_, context)?;
                let entry = self.get_deb_defining_entry(param.name.val());
                let decreasing_index_or_param_count = match &*param.deckw {
                    jnode::OptDecKw::None => DecIndexOrParamCount::ParamCount(1),
                    jnode::OptDecKw::Some(_) => DecIndexOrParamCount::DecIndex(0, param),
                };

                Ok((
                    vec![entry],
                    vec![param_type],
                    decreasing_index_or_param_count,
                ))
            }

            jnode::CommaSeparatedFunParamDefs::Snoc(rdc, rac) => {
                let (mut entries, mut param_types, rdc_decreasing_index) =
                    self.convert_typed_fun_param_defs_to_context_extension_aux(rdc, context)?;

                let extended_context = Context::Snoc(&context, &entries);
                let rac_type = self.convert(&rac.type_, extended_context)?;

                entries.push(self.get_deb_defining_entry(rac.name.val()));
                param_types.push(rac_type);

                let decreasing_index_or_param_count = match (rdc_decreasing_index, &*rac.deckw) {
                    (DecIndexOrParamCount::ParamCount(rdc_count), jnode::OptDecKw::None) => {
                        DecIndexOrParamCount::ParamCount(rdc_count + 1)
                    }

                    (DecIndexOrParamCount::ParamCount(rdc_count), jnode::OptDecKw::Some(_)) => {
                        DecIndexOrParamCount::DecIndex(rdc_count, rac)
                    }

                    (
                        DecIndexOrParamCount::DecIndex(rdc_index, rdc_param),
                        jnode::OptDecKw::None,
                    ) => DecIndexOrParamCount::DecIndex(rdc_index, rdc_param),

                    (DecIndexOrParamCount::DecIndex(_, rdc_param), jnode::OptDecKw::Some(_)) => {
                        return Err(SemanticError::MultipleDecreasingParams(
                            rdc_param.clone(),
                            *rac.clone(),
                        ))
                    }
                };
                Ok((entries, param_types, decreasing_index_or_param_count))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum DecIndexOrParamCount<'a> {
    DecIndex(usize, &'a jnode::FunParamDef),
    ParamCount(usize),
}

impl JuneConverter {
    pub(crate) fn get_context_extension_for_match_return_type_params<'a>(
        &mut self,
        matchee: &jnode::Expr,
        context: Context,
        arity_clause: &'a jnode::OptMatchReturnTypeClause,
    ) -> Result<Vec<UnshiftedEntry<'a>>, SemanticError> {
        match arity_clause {
            jnode::OptMatchReturnTypeClause::None => self
                .get_anonymous_context_extension_with_len_equal_to_match_return_arity(
                    matchee, context,
                ),

            jnode::OptMatchReturnTypeClause::Some(return_type_clause) => self
                .get_context_extension_for_match_return_type_params_using_match_return_type_clause(
                    matchee,
                    context,
                    return_type_clause,
                ),
        }
    }

    pub(crate) fn get_anonymous_context_extension_with_len_equal_to_match_return_arity(
        &mut self,
        matchee: &jnode::Expr,
        context: Context,
    ) -> Result<Vec<UnshiftedEntry<'static>>, SemanticError> {
        let return_arity = self.infer_match_return_arity(matchee, context)?;

        Ok((0..return_arity)
            .map(|_| self.get_deb_defining_entry("_"))
            .collect())
    }

    pub(crate) fn infer_match_return_arity<'a>(
        &mut self,
        matchee: &jnode::Expr,
        context: Context,
    ) -> Result<usize, SemanticError> {
        let matchee_type = self.convert_and_typecheck(matchee, context)?.type_;
        match matchee_type.raw() {
            znode::Expr::Ind(_) => Ok(1),

            znode::Expr::App(matchee_type_app) => match &matchee_type_app.hashee.callee {
                znode::Expr::Ind(_) => Ok(1 + matchee_type_app.hashee.args.hashee.len()),

                znode::Expr::Vcon(_)
                | znode::Expr::Match(_)
                | znode::Expr::Fun(_)
                | znode::Expr::App(_)
                | znode::Expr::For(_)
                | znode::Expr::Deb(_)
                | znode::Expr::Universe(_) => Err(SemanticError::MatcheeHasUnmatchableType(
                    matchee.clone(),
                    matchee_type,
                )),
            },

            znode::Expr::Vcon(_)
            | znode::Expr::Match(_)
            | znode::Expr::Fun(_)
            | znode::Expr::App(_)
            | znode::Expr::For(_)
            | znode::Expr::Deb(_)
            | znode::Expr::Universe(_) => Err(SemanticError::MatcheeHasUnmatchableType(
                matchee.clone(),
                matchee_type,
            )),
        }
    }

    fn get_context_extension_for_match_return_type_params_using_match_return_type_clause<'a>(
        &mut self,
        matchee: &jnode::Expr,
        context: Context,
        return_type_clause: &'a jnode::MatchReturnTypeClause,
    ) -> Result<Vec<UnshiftedEntry<'a>>, SemanticError> {
        Ok(match *return_type_clause.return_params {
            jnode::ReturnParamClause::None(_) => self
                .get_anonymous_context_extension_with_len_equal_to_match_return_arity(
                    matchee, context,
                )?,

            jnode::ReturnParamClause::Matchee(matchee_name_token, _) => {
                let return_arity = self.infer_match_return_arity(matchee, context)?;
                let return_arity_minus_1 = return_arity
                    .checked_sub(1)
                    .expect("Return arity should always be 1 or greater.");

                (0..return_arity_minus_1)
                    .map(|_| self.get_deb_defining_entry("_"))
                    .chain(std::iter::once(
                        self.get_deb_defining_entry(&matchee_name_token.value),
                    ))
                    .collect()
            }

            jnode::ReturnParamClause::Indices(index_name_tokens) => {
                // TODO: Calculate return_arity,
                // and then check that it is equal to index_names.len().
                index_name_tokens
                    .idents
                    .to_vec()
                    .into_iter()
                    .map(|name| self.get_deb_defining_entry(name.val()))
                    .collect()
            }

            jnode::ReturnParamClause::MatcheeAndIndices(matchee_name_token, index_name_tokens) => {
                // TODO: Calculate return_arity,
                // and then check that it is equal to index_names.len().
                index_name_tokens
                    .idents
                    .to_vec()
                    .into_iter()
                    .map(|name| self.get_deb_defining_entry(name.val()))
                    .chain(std::iter::once(
                        self.get_deb_defining_entry(&matchee_name_token.value),
                    ))
                    .collect()
            }
        })
    }
}

impl JuneConverter {
    pub(crate) fn convert_match_case_params_to_context_extension<'a>(
        &mut self,
        params: &'a jnode::OptParenthesizedCommaSeparatedIdentsOrUnderscores,
    ) -> Vec<UnshiftedEntry<'a>> {
        match params {
            jnode::OptParenthesizedCommaSeparatedIdentsOrUnderscores::None => vec![],

            jnode::OptParenthesizedCommaSeparatedIdentsOrUnderscores::Some(parenthesized) => {
                let mut out = vec![];
                self.push_index_names(&parenthesized.idents, &mut out);
                out
            }
        }
    }

    fn push_index_names<'a>(
        &mut self,
        index_names: &'a jnode::CommaSeparatedIdentsOrUnderscores,
        out: &mut Vec<UnshiftedEntry<'a>>,
    ) {
        match index_names {
            jnode::CommaSeparatedIdentsOrUnderscores::One(ident_or_underscore) => {
                out.push(self.get_deb_defining_entry(ident_or_underscore.val()));
            }

            jnode::CommaSeparatedIdentsOrUnderscores::Snoc(rdc, rac) => {
                self.push_index_names(rdc, out);
                out.push(self.get_deb_defining_entry(rac.val()));
            }
        }
    }
}

impl JuneConverter {
    pub(crate) fn get_deb_defining_entry<'a>(&mut self, key: &'a str) -> UnshiftedEntry<'a> {
        let val = self.cache_deb(znode::DebNode {
            deb: Deb(0),
            aux_data: (),
        });
        UnshiftedEntry {
            key,
            val,
            is_deb: true,
        }
    }
}
