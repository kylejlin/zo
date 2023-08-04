use super::*;

impl MayConverter {
    pub(crate) fn convert_let(
        &mut self,
        expr: &mnode::Let,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let val = self.convert(&expr.val, context)?;

        let val_singleton = [UnshiftedEntry {
            key: &expr.name.value,
            val,
            defines_deb: false,
        }];
        let extended_context = Context::Snoc(&context, &val_singleton);

        self.convert(&expr.next_val, extended_context)
    }
}
