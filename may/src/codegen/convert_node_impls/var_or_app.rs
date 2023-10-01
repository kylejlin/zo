use super::*;

impl MayConverter {
    pub(crate) fn convert_var_or_app<C: ContextToOwned>(
        &mut self,
        expr: &mnode::VarOrApp,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        match expr {
            mnode::VarOrApp::Var(e) => self.convert_var(e, context, converter),
            mnode::VarOrApp::App(e) => self.convert_app(e, context, converter),
        }
    }

    fn convert_var<C: ContextToOwned>(
        &mut self,
        expr: &mnode::Ident,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let Some((entry, Distance(dist))) = context.get(&expr.value) else {
            return Err(SemanticError::VarNotDefined(expr.clone()));
        };
        let val = entry.val.clone().replace_debs(&DebUpshifter(dist), 0);
        let converted_leaf = self.cache_expr(val);
        Ok((converted_leaf, converter.convert_context_to_owned(context)))
    }

    fn convert_app<C: ContextToOwned>(
        &mut self,
        expr: &mnode::App,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let (callee, _) = self.convert_var_or_app(&expr.callee, context, &DropContext)?;
        let args = self.convert_exprs(&expr.args, context)?;
        let converted_leaf = self.cache_app(znode::App {
            callee,
            args,
            aux_data: (),
        });
        Ok((converted_leaf, converter.convert_context_to_owned(context)))
    }
}
