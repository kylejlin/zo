use super::*;

impl JuneConverter {
    pub(crate) fn convert_chain_var_def(
        &mut self,
        expr: &mnode::ChainVarDef,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let val = self.convert(&expr.standalone.val, context)?;

        let val_singleton = [UnshiftedEntry {
            key: &expr.standalone.name.value,
            val,
            def_type: DefinitionType::Substitutable,
        }];
        let extended_context = Context::Snoc(&context, &val_singleton);

        self.convert(&expr.next_val, extended_context)
    }
}
