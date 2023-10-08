use super::*;

impl JuneConverter {
    pub(crate) fn convert_fun(
        &mut self,
        expr: &mnode::Fun,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let converted_leaf =
            self.convert_fun_innards(&expr.innards, expr.name.val_or_underscore(), context)?;
        Ok(converted_leaf)
    }

    pub(crate) fn convert_fun_innards(
        &mut self,
        expr: &mnode::FunCommonInnards,
        fun_name: &str,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (extension, param_types, decreasing_index) =
            self.convert_typed_fun_param_defs_to_context_extension(&expr.params.params, context)?;
        let context_with_params = Context::Snoc(&context, &extension);

        let return_type = self.convert(&expr.return_type, context_with_params)?;

        let recursive_fun_param_singleton = [self.get_deb_defining_entry(fun_name)];

        let context_with_recursive_fun_param =
            Context::Snoc(&context_with_params, &recursive_fun_param_singleton);

        let return_val = self.convert(&expr.return_val, context_with_recursive_fun_param)?;

        let param_types = self.cache_expr_vec(param_types);

        Ok(self.cache_fun(znode::Fun {
            decreasing_index,
            param_types,
            return_type,
            return_val,
            aux_data: (),
        }))
    }
}
