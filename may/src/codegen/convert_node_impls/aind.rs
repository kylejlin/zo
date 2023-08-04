use super::*;

impl MayConverter {
    pub(crate) fn convert_aind(
        &mut self,
        expr: &mnode::Aind,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match &*expr.innards.params {
            mnode::OptParenthesizedParamDefs::None => {
                self.convert_unparameterized_aind(&expr.innards, context)
            }

            mnode::OptParenthesizedParamDefs::Some(parenthesized) => todo!(),
        }
    }

    fn convert_unparameterized_aind(
        &mut self,
        expr: &mnode::IndCommonInnards,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let universe_level = UniverseLevel(expr.universe.level);

        let name = match &*expr.custom_zo_name {
            mnode::OptString::None => StringValue(expr.name.value.clone()),
            mnode::OptString::Some(string_literal) => StringValue(string_literal.value.clone()),
        };
        let name = self.cache_string_value(name);

        let (_, index_types, _) = self.convert_optional_typed_param_defs_to_context_extension(
            expr.indices.defs(),
            context,
            ForbidDash,
        )?;
        let index_types = self.cache_expr_vec(index_types);

        let recursive_ind_param_singleton = [self.get_deb_defining_entry(&expr.name.value)];
        let context_with_recursive_ind = Context::Snoc(&context, &recursive_ind_param_singleton);
        let vcon_defs = self.convert_ind_cases(&expr.cases, context_with_recursive_ind)?;

        Ok(self.cache_ind(znode::Ind {
            universe_level,
            name,
            index_types,
            vcon_defs,
        }))
    }

    /// `context` should already contain the recursive ind entry.
    fn convert_ind_cases(
        &mut self,
        expr: &mnode::ZeroOrMoreIndCases,
        context: Context,
    ) -> Result<RcHashedVec<znode::VconDef>, SemanticError> {
        todo!()
    }
}
