use super::*;

impl JuneConverter {
    pub(crate) fn convert_chain_def(
        &mut self,
        expr: &mnode::ChainDef,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let fun = self.convert_fun_innards(
            &expr.standalone.innards,
            &expr.standalone.name.value,
            context,
        )?;
        let fun_singleton = [UnshiftedEntry {
            key: &expr.standalone.name.value,
            val: fun,
            def_type: DefinitionType::Substitutable,
        }];
        let context_with_fun = Context::Snoc(&context, &fun_singleton);
        self.convert(&expr.next_val, context_with_fun)
    }
}
