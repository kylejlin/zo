use super::*;

impl JuneConverter {
    pub(crate) fn convert_chain_var_def(
        &mut self,
        expr: &jnode::ChainVarDef,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let val = self.convert_and_typecheck(&expr.standalone.val, context)?;

        let val_singleton = [UnshiftedEntry {
            key: &expr.standalone.name.value,
            val,
            is_deb: false,
        }];
        let extended_context = Context::Snoc(&context, &val_singleton);

        self.convert(&expr.next_val, extended_context)
    }
}

impl JuneConverter {
    pub fn convert_mod_and_typecheck(
        &mut self,
        expr: &jnode::Module,
        context: Context,
    ) -> Result<Vec<(String, znode::Expr)>, SemanticError> {
        todo!()
    }
}
