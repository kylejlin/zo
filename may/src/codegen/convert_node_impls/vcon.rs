use super::*;

impl MayConverter {
    pub(crate) fn convert_vcon(
        &mut self,
        expr: &mnode::Vcon,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let vcon_index = expr.vcon_index.index;
        if vcon_index >= expr.innards.cases.len() {
            return Err(SemanticError::InvalidVconIndex(expr.vcon_index.clone()));
        }

        self.convert_vcon_with_valid_vcon_index(&expr.innards, vcon_index, context)
    }

    pub(crate) fn convert_vcon_with_valid_vcon_index(
        &mut self,
        expr: &mnode::IndCommonInnards,
        valid_vcon_index: usize,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match &*expr.params {
            mnode::OptParenthesizedParamDefs::None => self
                .convert_unparameterized_vcon_with_valid_vcon_index(
                    expr,
                    valid_vcon_index,
                    context,
                ),

            mnode::OptParenthesizedParamDefs::Some(parenthesized) => self
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
        expr: &mnode::IndCommonInnards,
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
        }))
    }

    fn convert_parameterized_vcon_with_valid_vcon_index(
        &mut self,
        expr: &mnode::IndCommonInnards,
        valid_vcon_index: usize,
        params: &mnode::CommaSeparatedParamDefs,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (param_def_entries, param_types, ()) =
            self.convert_typed_param_defs_to_context_extension(params, context, ForbidDash)?;

        let param_types = self.cache_expr_vec(param_types);

        let param_def_entries_and_unusable_recursive_fun_entry = {
            let mut out = param_def_entries;
            out.push(self.get_deb_defining_entry("_"));
            out
        };
        let context_with_params = Context::Snoc(
            &context,
            &param_def_entries_and_unusable_recursive_fun_entry,
        );
        let ind = self.convert_unparameterized_ind_innards_to_zo_ind(&expr, context_with_params)?;
        let ind = self
            .cache_ind(ind)
            .try_into_ind()
            .expect("cache_ind should always return (the same) ind");

        let normalized_ind = self.zo_typechecker.evaluator.eval_ind(ind.clone());
        let return_type = self
            .zo_typechecker
            .get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(
                normalized_ind,
                valid_vcon_index,
            )
            .into_raw();

        let return_val = self.cache_vcon(znode::Vcon {
            ind,
            vcon_index: valid_vcon_index,
        });

        Ok(self.cache_fun(znode::Fun {
            decreasing_index: None,
            param_types,
            return_type,
            return_val,
        }))
    }
}
