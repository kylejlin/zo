use super::*;

impl MayConverter {
    pub(crate) fn convert_let<C: ContextToOwned>(
        &mut self,
        expr: &mnode::Let,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let (val, _) = self.convert(&expr.val, context, &DropContext)?;

        let val_singleton = [UnshiftedEntry {
            key: &expr.name.value,
            val,
            def_type: DefinitionType::Substitutable,
        }];
        let extended_context = Context::Snoc(&context, &val_singleton);

        self.convert(&expr.next_val, extended_context, converter)
    }
}
