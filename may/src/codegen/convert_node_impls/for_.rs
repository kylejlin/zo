use super::*;

impl MayConverter {
    pub(crate) fn convert_for(
        &mut self,
        expr: &mnode::For,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (extension, param_types, ()) = self.convert_typed_param_defs_to_context_extension(
            &expr.params.params,
            context,
            ForbidDash,
        )?;
        let extended_context = Context::Snoc(&context, &extension);
        let return_type = self.convert(&expr.return_type, extended_context)?;

        let param_types = self.cache_expr_vec(param_types);

        Ok(self.cache_for(znode::For {
            param_types,
            return_type,
        }))
    }
}
