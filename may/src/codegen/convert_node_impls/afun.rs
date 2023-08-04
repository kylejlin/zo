use super::*;

impl MayConverter {
    pub(crate) fn convert_afun(
        &mut self,
        expr: &mnode::Afun,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (extension, param_types, decreasing_index) = self
            .convert_param_defs_to_context_extension(
                &expr.innards.params.params,
                context,
                AtMostOneDash::default(),
            )?;
        let context_with_params = Context::Snoc(&context, &extension);

        let return_type = self.convert(&expr.innards.return_type, context_with_params)?;

        let recursive_fun_param_singleton =
            [self.get_deb_defining_entry(expr.name.val_or_underscore())];

        let context_with_recursive_fun_param =
            Context::Snoc(&context_with_params, &recursive_fun_param_singleton);

        let return_val =
            self.convert(&expr.innards.return_val, context_with_recursive_fun_param)?;

        let param_types = self.cache_expr_vec(param_types);

        Ok(self.cache_fun(znode::Fun {
            decreasing_index,
            param_types,
            return_type,
            return_val,
        }))
    }
}
