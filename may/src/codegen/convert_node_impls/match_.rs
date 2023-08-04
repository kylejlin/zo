use super::*;

impl MayConverter {
    pub(crate) fn convert_match(
        &mut self,
        expr: &mnode::Match,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let matchee = self.convert(&expr.matchee, context)?;

        let extension =
            self.convert_return_arity_clause_to_context_extension(&expr.return_arity)?;
        let context_with_return_params = Context::Snoc(&context, &extension);
        let return_type = self.convert(&expr.return_type, context_with_return_params)?;

        let cases = self.convert_match_cases(&expr.cases, context)?;

        Ok(self.cache_match(znode::Match {
            matchee,
            return_type,
            cases,
        }))
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

        let return_val = self.convert(&case.return_val, context_with_params)?;

        Ok(znode::MatchCase { arity, return_val })
    }
}
