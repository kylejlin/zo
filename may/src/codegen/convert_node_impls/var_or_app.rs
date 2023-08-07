use super::*;

impl MayConverter {
    pub(crate) fn convert_var_or_app(
        &mut self,
        expr: &mnode::VarOrApp,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match expr {
            mnode::VarOrApp::Var(e) => self.convert_var(e, context),
            mnode::VarOrApp::App(e) => self.convert_app(e, context),
        }
    }

    fn convert_var(
        &mut self,
        expr: &mnode::Ident,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let Some((entry, Distance(dist))) = context.get(&expr.value) else {
            return Err(SemanticError::VarNotDefined(expr.clone()));
        };
        let val = entry.val.clone().replace_debs(&DebUpshifter(dist), 0);
        {
            //_! TODO: Delete this block.
            use zoc::pretty_print::PrettyPrint;
            if expr.value == "add_n_succ_m" {
                println!("XXX.add_n_succ_m(dist={dist}):\n{}", PrettyPrint(&val));
            }
        }
        Ok(self.cache_expr(val))
    }

    fn convert_app(
        &mut self,
        expr: &mnode::App,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let callee = self.convert_var_or_app(&expr.callee, context)?;
        let args = self.convert_exprs(&expr.args, context)?;
        Ok(self.cache_app(znode::App { callee, args }))
    }
}
