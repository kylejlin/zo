use super::*;

impl MayConverter {
    pub(crate) fn convert_ind<C: ContextToOwned>(
        &mut self,
        expr: &mnode::Ind,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let ind = self.convert_ind_innards(&expr.innards, context)?;

        let vcon_extension = self.get_vcon_definitions(expr, context)?;

        let ind_singleton = [UnshiftedEntry {
            key: &expr.innards.name.value,
            val: ind,
            def_type: DefinitionType::Substitutable,
        }];
        let context_with_ind = Context::Snoc(&context, &ind_singleton);
        let context_with_ind_and_vcons = Context::Snoc(&context_with_ind, &vcon_extension);

        self.convert(&expr.next_val, context_with_ind_and_vcons, converter)
    }

    fn get_vcon_definitions<'a>(
        &mut self,
        expr: &'a mnode::Ind,
        context: Context,
    ) -> Result<Vec<UnshiftedEntry<'a>>, SemanticError> {
        let mut cases = expr.innards.cases.to_vec();
        cases.sort_by(|a, b| a.name.cmp(&b.name));
        cases
            .into_iter()
            .enumerate()
            .map(|(index, case)| {
                let vcon =
                    self.convert_vcon_with_valid_vcon_index(&expr.innards, index, context)?;
                Ok(UnshiftedEntry {
                    key: &case.name.value,
                    val: vcon,
                    def_type: DefinitionType::Substitutable,
                })
            })
            .collect()
    }
}
