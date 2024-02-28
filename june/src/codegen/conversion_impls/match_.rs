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

        let return_type = self.get_match_return_type(expr, context)?;

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

    fn get_match_return_type(
        &mut self,
        expr: &jnode::Match,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        match &*expr.return_type {
            jnode::OptMatchReturnTypeClause::None => self.get_first_case_return_type(expr, context),

            jnode::OptMatchReturnTypeClause::Some(return_type_clause) => {
                let extension = self.get_context_extension_for_match_return_type_params(
                    &expr.matchee,
                    context,
                    &expr.return_type,
                )?;
                let extended_context = Context::Snoc(&context, &extension);
                self.convert(&return_type_clause.return_type, extended_context)
            }
        }
    }

    fn get_first_case_return_type(
        &mut self,
        expr: &jnode::Match,
        context: Context,
    ) -> Result<znode::Expr, SemanticError> {
        todo!()
        // Difficult
    }
}
