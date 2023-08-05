use super::*;

impl MayConverter {
    pub(crate) fn convert_ind(
        &mut self,
        expr: &mnode::Ind,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let ind = self.convert_ind_innards(&expr.innards, context)?;

        let vcon_extension = self.get_vcon_definitions(expr, context)?;

        let ind_singleton = [UnshiftedEntry {
            key: &expr.innards.name.value,
            val: ind,
            defines_deb: false,
        }];
        let context_with_ind = Context::Snoc(&context, &ind_singleton);
        let context_with_ind_and_vcons = Context::Snoc(&context_with_ind, &vcon_extension);

        self.convert(&expr.next_val, context_with_ind_and_vcons)
    }

    fn get_vcon_definitions<'a>(
        &mut self,
        expr: &'a mnode::Ind,
        context: Context,
    ) -> Result<Vec<UnshiftedEntry<'a>>, SemanticError> {
        todo!()

        // let mut cases = expr.innards.cases.to_vec();
        // cases.sort_by(|a, b| a.name.cmp(&b.name));
    }
}
