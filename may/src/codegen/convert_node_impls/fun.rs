use super::*;

impl MayConverter {
    pub(crate) fn convert_fun(
        &mut self,
        expr: &mnode::Fun,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let fun = self.convert_fun_common_innards(&expr.innards, &expr.name.value, context)?;
        let fun_singleton = [UnshiftedEntry {
            key: &expr.name.value,
            val: fun,
            defines_deb: false,
        }];
        let context_with_fun = Context::Snoc(&context, &fun_singleton);
        self.convert(&expr.next_val, context_with_fun)
    }
}
