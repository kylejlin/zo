use super::*;

impl JuneConverter {
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
