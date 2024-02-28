use super::*;

impl JuneConverter {
    pub(crate) fn convert_match(
        &mut self,
        expr: &jnode::Match,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        let matchee = self.convert(&expr.matchee, context)?;

        let extension = self.get_context_extension_for_match_return_type_params(
            &expr.matchee,
            context,
            &expr.return_type,
        )?;
        let return_type_arity = extension.len();
        let context_with_return_params = Context::Snoc(&context, &extension);

        // TODO: Delete
        // let return_type = self.convert(&expr.return_type, context_with_return_params)?;
        // TODO: Generate return type if it exists. Otherwise, infer it.

        let cases = self.convert_match_cases(&expr.cases, context)?;

        let converted_leaf = self.cache_match(znode::Match {
            matchee,
            return_type_arity,
            return_type,
            cases,
            aux_data: (),
        });
        Ok(converted_leaf)
    }

    // TODO: Check that the correct cases are covered.
    // Currently, case names are ignored.
    // For example, you can legally match a nat against `b` and `a(foo)`,
    // since that is isomorphic to `zero` and `succ(pred)` (assuming alphabetical sorting).
    fn convert_match_cases(
        &mut self,
        cases: &jnode::ZeroOrMoreMatchCases,
        context: Context,
    ) -> Result<RcHashedVec<znode::MatchCase>, SemanticError> {
        let mut cases = cases.to_vec();
        cases.sort_unstable_by(|a, b| a.name.value.cmp(&b.name.value));
        self.convert_ordered_match_cases(&cases, context)
    }

    fn convert_ordered_match_cases(
        &mut self,
        cases: &[&jnode::MatchCase],
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
        case: &jnode::MatchCase,
        context: Context,
    ) -> Result<znode::MatchCase, SemanticError> {
        let arity = case.params.len();

        let extension = self.convert_match_case_params_to_context_extension(&case.params);
        let context_with_params = Context::Snoc(&context, &extension);

        let return_val = self.convert(&case.return_val, context_with_params)?;

        Ok(znode::MatchCase {
            arity,
            return_val,
            aux_data: (),
        })
    }
}
