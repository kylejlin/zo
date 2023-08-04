use super::*;

impl MayConverter {
    pub(crate) fn convert_vcon(
        &mut self,
        expr: &mnode::Vcon,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match &*expr.innards.params {
            mnode::OptParenthesizedParamDefs::None => {
                self.convert_unparameterized_vcon(expr, context)
            }

            mnode::OptParenthesizedParamDefs::Some(parenthesized) => {
                self.convert_parameterized_vcon(expr, &parenthesized.params, context)
            }
        }
    }

    fn convert_unparameterized_vcon(
        &mut self,
        expr: &mnode::Vcon,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let ind = self.convert_unparameterized_ind_innards_to_zo_ind(&expr.innards, context)?;
        let ind = self
            .cache_ind(ind)
            .try_into_ind()
            .expect("cache_ind should always return (the same) ind");

        let vcon_index = expr.vcon_index.index;

        Ok(self.cache_vcon(znode::Vcon { ind, vcon_index }))
    }

    fn convert_parameterized_vcon(
        &mut self,
        expr: &mnode::Vcon,
        params: &mnode::CommaSeparatedParamDefs,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (extension, param_types, ()) =
            self.convert_typed_param_defs_to_context_extension(params, context, ForbidDash)?;

        let param_types = self.cache_expr_vec(param_types);

        let context_with_params = Context::Snoc(&context, &extension);
        let ind =
            self.convert_unparameterized_ind_innards_to_zo_ind(&expr.innards, context_with_params)?;
        let ind = self
            .cache_ind(ind)
            .try_into_ind()
            .expect("cache_ind should always return (the same) ind");

        let vcon_index = expr.vcon_index.index;
        let normalized_ind = self.zo_typechecker.evaluator.eval_ind(ind.clone());
        let return_type = self
            .zo_typechecker
            .get_type_of_vcon_from_well_typed_ind_and_valid_vcon_index(normalized_ind, vcon_index)
            .into_raw();

        let return_val = self.cache_vcon(znode::Vcon { ind, vcon_index });

        Ok(self.cache_fun(znode::Fun {
            decreasing_index: None,
            param_types,
            return_type,
            return_val,
        }))
    }
}
