use super::*;

impl MayConverter {
    pub(crate) fn convert_match<C: ContextToOwned>(
        &mut self,
        expr: &mnode::Match,
        context: Context,
        converter: &C,
    ) -> Result<(znode::Expr, C::Out), SemanticError> {
        let (matchee, _) = self.convert(&expr.matchee, context, &DropContext)?;

        let extension =
            self.convert_return_arity_clause_to_context_extension(&expr.return_arity)?;
        let return_type_arity = extension.len();
        let context_with_return_params = Context::Snoc(&context, &extension);
        let (return_type, _) =
            self.convert(&expr.return_type, context_with_return_params, &DropContext)?;

        let cases = self.convert_match_cases(&expr.cases, context)?;

        let converted_leaf = self.cache_match(znode::Match {
            matchee,
            return_type_arity,
            return_type,
            cases,
            aux_data: (),
        });
        Ok((converted_leaf, converter.convert_context_to_owned(context)))
    }

    fn convert_match_cases(
        &mut self,
        cases: &mnode::ZeroOrMoreMatchCases,
        context: Context,
    ) -> Result<RcHashedVec<znode::MatchCase>, SemanticError> {
        let mut cases = cases.to_vec();
        cases.sort_unstable_by(|a, b| a.name.value.cmp(&b.name.value));
        self.convert_ordered_match_cases(&cases, context)
    }

    fn convert_ordered_match_cases(
        &mut self,
        cases: &[&mnode::MatchCase],
        context: Context,
    ) -> Result<RcHashedVec<znode::MatchCase>, SemanticError> {
        let v: Vec<znode::MatchCase> = cases
            .into_iter()
            .map(|case| self.convert_match_case(case, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(bypass_cache_and_rc_hash(v))
    }

    fn convert_match_case(
        &mut self,
        case: &mnode::MatchCase,
        context: Context,
    ) -> Result<znode::MatchCase, SemanticError> {
        let arity = case.params.len();

        let extension = self.convert_match_case_params_to_context_extension(&case.params);
        let context_with_params = Context::Snoc(&context, &extension);

        let (return_val, _) = self.convert(&case.return_val, context_with_params, &DropContext)?;

        Ok(znode::MatchCase {
            arity,
            return_val,
            aux_data: (),
        })
    }
}
