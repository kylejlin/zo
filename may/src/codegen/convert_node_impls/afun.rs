use super::*;

impl MayConverter {
    pub(crate) fn convert_afun<C: ContextToOwned>(
        &mut self,
        expr: &mnode::Afun,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let converted_leaf =
            self.convert_fun_innards(&expr.innards, expr.name.val_or_underscore(), context)?;
        Ok((converted_leaf, converter.convert_context_to_owned(context)))
    }

    pub(crate) fn convert_fun_innards(
        &mut self,
        expr: &mnode::FunCommonInnards,
        fun_name: &str,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (extension, param_types, decreasing_index) = self
            .convert_typed_param_defs_to_context_extension(
                &expr.params.params,
                context,
                AtMostOneDash::default(),
            )?;
        let context_with_params = Context::Snoc(&context, &extension);

        let (return_type, _) =
            self.convert(&expr.return_type, context_with_params, &DropContext)?;

        let recursive_fun_param_singleton = [self.get_deb_defining_entry(fun_name)];

        let context_with_recursive_fun_param =
            Context::Snoc(&context_with_params, &recursive_fun_param_singleton);

        let (return_val, _) = self.convert(
            &expr.return_val,
            context_with_recursive_fun_param,
            &DropContext,
        )?;

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
