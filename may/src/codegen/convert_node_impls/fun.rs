use super::*;

impl MayConverter {
    pub(crate) fn convert_fun<C: ContextToOwned>(
        &mut self,
        expr: &mnode::Fun,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let fun = self.convert_fun_innards(&expr.innards, &expr.name.value, context)?;
        let fun_singleton = [UnshiftedEntry {
            key: &expr.name.value,
            val: fun,
            def_type: DefinitionType::Substitutable,
        }];
        let context_with_fun = Context::Snoc(&context, &fun_singleton);
        self.convert(&expr.next_val, context_with_fun, converter)
    }
}
