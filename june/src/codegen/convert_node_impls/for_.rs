use super::*;

impl JuneConverter {
    pub(crate) fn convert_for(
        &mut self,
        expr: &mnode::For,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let (extension, param_types) = self
            .convert_typed_nonfun_param_defs_to_context_extension(&expr.params.params, context)?;
        let extended_context = Context::Snoc(&context, &extension);
        let return_type = self.convert(&expr.return_type, extended_context)?;

        let param_types = self.cache_expr_vec(param_types);

        let converted_leaf = self.cache_for(znode::For {
            param_types,
            return_type,
            aux_data: (),
        });
        Ok(converted_leaf)
    }
}
