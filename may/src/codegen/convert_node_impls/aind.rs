use super::*;

impl MayConverter {
    pub(crate) fn convert_aind<C: ContextToOwned>(
        &mut self,
        expr: &mnode::Aind,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let converted_leaf = self.convert_ind_innards(&expr.innards, context)?;
        Ok((converted_leaf, converter.convert_context_to_owned(context)))
    }

    pub(crate) fn convert_ind_innards(
        &mut self,
        expr: &mnode::IndCommonInnards,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match &*expr.params {
            mnode::OptParenthesizedParamDefs::None => {
                self.convert_unparameterized_ind_innards(&expr, context)
            }

            mnode::OptParenthesizedParamDefs::Some(parenthesized) => {
                self.convert_parameterized_ind_innards(&expr, &parenthesized.params, context)
            }
        }
    }

    fn convert_unparameterized_ind_innards(
        &mut self,
        expr: &mnode::IndCommonInnards,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let ind = self.convert_unparameterized_ind_innards_to_zo_ind(expr, context)?;
        Ok(self.cache_ind(ind))
    }

    pub(crate) fn convert_unparameterized_ind_innards_to_zo_ind(
        &mut self,
        expr: &mnode::IndCommonInnards,
        context: Context,
    ) -> Result<znode::Ind, SemanticError> {
        let universe_level = Universe {
            level: UniverseLevel(expr.universe.level),
            erasable: expr.universe.erasable,
        };

        let name = match &*expr.custom_zo_name {
            mnode::OptString::None => StringValue(expr.name.value.clone()),
            mnode::OptString::Some(string_literal) => StringValue(string_literal.value.clone()),
        };
        let name = self.cache_string_value(name);

        let (_, index_types, _) = self.convert_optional_typed_param_defs_to_context_extension(
            expr.indices.to_std_option(),
            context,
            ForbidDash,
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
        cases: &[&mnode::IndCase],
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
        case: &mnode::IndCase,
        context: Context,
    ) -> Result<znode::VconDef, SemanticError> {
        let (extension, param_types, _) = self
            .convert_optional_typed_param_defs_to_context_extension(
                case.params.to_std_option(),
                context,
                ForbidDash,
            )?;
        let param_types = self.cache_expr_vec(param_types);

        let extended_context = Context::Snoc(&context, &extension);
        let index_args =
            self.convert_optional_exprs(case.return_type.to_std_option(), extended_context)?;

        Ok(znode::VconDef {
            param_types,
            index_args,
            aux_data: (),
        })
    }

    fn convert_parameterized_ind_innards(
        &mut self,
        expr: &mnode::IndCommonInnards,
        params: &mnode::CommaSeparatedParamDefs,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (param_def_entries, param_types, ()) =
            self.convert_typed_param_defs_to_context_extension(params, context, ForbidDash)?;

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
