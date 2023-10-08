use super::*;

impl JuneConverter {
    pub(crate) fn convert_chain_enum(
        &mut self,
        expr: &mnode::ChainEnum,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let ind = self.convert_ind_innards(&expr.standalone, context)?;

        let vcon_extension = self.get_vcon_definitions(&expr.standalone, context)?;

        let ind_singleton = [UnshiftedEntry {
            key: &expr.standalone.name.value,
            val: ind,
            def_type: DefinitionType::Substitutable,
        }];
        let context_with_ind = Context::Snoc(&context, &ind_singleton);
        let context_with_ind_and_vcons = Context::Snoc(&context_with_ind, &vcon_extension);

        self.convert(&expr.next_val, context_with_ind_and_vcons)
    }

    fn get_vcon_definitions<'a>(
        &mut self,
        expr: &'a mnode::Enum,
        context: Context,
    ) -> Result<Vec<UnshiftedEntry<'a>>, SemanticError> {
        let mut cases = expr.cases.to_vec();
        cases.sort_by(|a, b| a.name.cmp(&b.name));
        cases
            .into_iter()
            .enumerate()
            .map(|(index, case)| {
                let vcon = self.convert_vcon_with_valid_vcon_index(&expr, index, context)?;
                Ok(UnshiftedEntry {
                    key: &case.name.value,
                    val: vcon,
                    def_type: DefinitionType::Substitutable,
                })
            })
            .collect()
    }
}

impl JuneConverter {
    fn convert_vcon_with_valid_vcon_index(
        &mut self,
        expr: &mnode::Enum,
        valid_vcon_index: usize,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match &*expr.params {
            mnode::OptParenthesizedNonfunParamDefs::None => self
                .convert_unparameterized_vcon_with_valid_vcon_index(
                    expr,
                    valid_vcon_index,
                    context,
                ),

            mnode::OptParenthesizedNonfunParamDefs::Some(parenthesized) => self
                .convert_parameterized_vcon_with_valid_vcon_index(
                    expr,
                    valid_vcon_index,
                    &parenthesized.params,
                    context,
                ),
        }
    }

    fn convert_unparameterized_vcon_with_valid_vcon_index(
        &mut self,
        expr: &mnode::Enum,
        valid_vcon_index: usize,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let ind = self.convert_unparameterized_ind_innards_to_zo_ind(&expr, context)?;
        let ind = self
            .cache_ind(ind)
            .try_into_ind()
            .expect("cache_ind should always return (the same) ind");

        Ok(self.cache_vcon(znode::Vcon {
            ind,
            vcon_index: valid_vcon_index,
            aux_data: (),
        }))
    }

    fn convert_parameterized_vcon_with_valid_vcon_index(
        &mut self,
        expr: &mnode::Enum,
        valid_vcon_index: usize,
        params: &mnode::CommaSeparatedNonfunParamDefs,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (param_def_entries, param_types) =
            self.convert_typed_nonfun_param_defs_to_context_extension(params, context)?;

        let param_types = self.cache_expr_vec(param_types);

        let context_with_params = Context::Snoc(&context, &param_def_entries);
        let return_type_ind =
            self.convert_unparameterized_ind_innards_to_zo_ind(&expr, context_with_params)?;
        let return_type_ind = self
            .cache_ind(return_type_ind)
            .try_into_ind()
            .expect("cache_ind should always return (the same) ind");
        let return_type_ind = self
            .zo_typechecker
            .evaluator
            .eval_ind(return_type_ind.clone());
        let return_type = self
            .zo_typechecker
            .get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
                return_type_ind,
                valid_vcon_index,
            )
            .into_raw();

        let unusable_singleton = [self.get_deb_defining_entry("_")];
        let context_with_params_and_recursive_fun =
            Context::Snoc(&context_with_params, &unusable_singleton);
        let return_val_ind = self.convert_unparameterized_ind_innards_to_zo_ind(
            &expr,
            context_with_params_and_recursive_fun,
        )?;
        let return_val_ind = self
            .cache_ind(return_val_ind)
            .try_into_ind()
            .expect("cache_ind should always return (the same) ind");

        let return_val = self.cache_vcon(znode::Vcon {
            ind: return_val_ind,
            vcon_index: valid_vcon_index,
            aux_data: (),
        });

        Ok(self.cache_fun(znode::Fun {
            decreasing_index: None,
            param_types,
            return_type,
            return_val,
            aux_data: (),
        }))
    }
}

impl JuneConverter {
    pub(crate) fn convert_ind_innards(
        &mut self,
        expr: &mnode::Enum,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match &*expr.params {
            mnode::OptParenthesizedNonfunParamDefs::None => {
                self.convert_unparameterized_ind_innards(&expr, context)
            }

            mnode::OptParenthesizedNonfunParamDefs::Some(parenthesized) => {
                self.convert_parameterized_ind_innards(&expr, &parenthesized.params, context)
            }
        }
    }

    fn convert_unparameterized_ind_innards(
        &mut self,
        expr: &mnode::Enum,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let ind = self.convert_unparameterized_ind_innards_to_zo_ind(expr, context)?;
        Ok(self.cache_ind(ind))
    }

    pub(crate) fn convert_unparameterized_ind_innards_to_zo_ind(
        &mut self,
        expr: &mnode::Enum,
        context: Context,
    ) -> Result<znode::Ind, SemanticError> {
        let universe_level = Universe {
            level: UniverseLevel(expr.universe.level),
            erasable: expr.universe.erasable,
        };

        let name = StringValue(expr.name.value.clone());
        let name = self.cache_string_value(name);

        let (_, index_types) = self.convert_optional_typed_nonfun_param_defs_to_context_extension(
            expr.indices.to_std_option(),
            context,
        )?;
        let index_types = self.cache_expr_vec(index_types);

        let recursive_ind_param_singleton = [self.get_deb_defining_entry(&expr.name.value)];
        let context_with_recursive_ind = Context::Snoc(&context, &recursive_ind_param_singleton);
        let mut cases = expr.cases.to_vec();
        cases.sort_unstable_by(|a, b| a.name.value.cmp(&b.name.value));
        let vcon_defs = self.convert_ordered_ind_cases(&cases, context_with_recursive_ind)?;

        Ok(znode::Ind {
            universe: universe_level,
            name,
            index_types,
            vcon_defs,
            aux_data: (),
        })
    }

    /// `context` should already contain the recursive ind entry.
    fn convert_ordered_ind_cases(
        &mut self,
        cases: &[&mnode::EnumCase],
        context: Context,
    ) -> Result<RcHashedVec<znode::VconDef>, SemanticError> {
        let v: Vec<znode::VconDef> = cases
            .into_iter()
            .map(|case| self.convert_ind_case(case, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(bypass_cache_and_rc_hash(v))
    }

    fn convert_ind_case(
        &mut self,
        case: &mnode::EnumCase,
        context: Context,
    ) -> Result<znode::VconDef, SemanticError> {
        let (extension, param_types) = self
            .convert_optional_typed_nonfun_param_defs_to_context_extension(
                case.params.to_std_option(),
                context,
            )?;
        let param_types = self.cache_expr_vec(param_types);

        let extended_context = Context::Snoc(&context, &extension);
        let index_args =
            self.convert_optional_exprs(case.index_args.to_std_option(), extended_context)?;

        Ok(znode::VconDef {
            param_types,
            index_args,
            aux_data: (),
        })
    }

    fn convert_parameterized_ind_innards(
        &mut self,
        expr: &mnode::Enum,
        params: &mnode::CommaSeparatedNonfunParamDefs,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (param_def_entries, param_types) =
            self.convert_typed_nonfun_param_defs_to_context_extension(params, context)?;

        let param_types = self.cache_expr_vec(param_types);

        let context_with_params = Context::Snoc(&context, &param_def_entries);
        let return_type_ind =
            self.convert_unparameterized_ind_innards_to_zo_ind(expr, context_with_params)?;

        let ind_type_cfor = znode::For {
            param_types: return_type_ind.index_types.clone(),
            return_type: znode::UniverseNode {
                universe: return_type_ind.universe,
                aux_data: (),
            }
            .into(),
            aux_data: (),
        }
        .collapse_if_nullary();

        let unusable_singleton = [self.get_deb_defining_entry("_")];
        let context_with_params_and_recursive_fun =
            Context::Snoc(&context_with_params, &unusable_singleton);
        let return_val = self.convert_unparameterized_ind_innards_to_zo_ind(
            expr,
            context_with_params_and_recursive_fun,
        )?;

        let return_val = self.cache_ind(return_val);

        Ok(self.cache_fun(znode::Fun {
            decreasing_index: None,
            param_types,
            return_type: ind_type_cfor,
            return_val,
            aux_data: (),
        }))
    }
}
