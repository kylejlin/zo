use super::*;

impl MayConverter {
    pub(crate) fn convert_for<C: ContextToOwned>(
        &mut self,
        expr: &mnode::For,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let (extension, param_types, ()) = self.convert_typed_param_defs_to_context_extension(
            &expr.params.params,
            context,
            ForbidDash,
        )?;
        let extended_context = Context::Snoc(&context, &extension);
        let (return_type, _) = self.convert(&expr.return_type, extended_context, &DropContext)?;

        let param_types = self.cache_expr_vec(param_types);

        let converted_leaf = self.cache_for(znode::For {
            param_types,
            return_type,
            aux_data: (),
        });
        Ok((converted_leaf, converter.convert_context_to_owned(context)))
    }
}
