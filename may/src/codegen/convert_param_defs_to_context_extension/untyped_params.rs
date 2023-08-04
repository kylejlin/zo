use super::*;

impl MayConverter {
    pub(crate) fn convert_return_arity_clause_to_context_extension<'a>(
        &mut self,
        arity_clause: &'a mnode::ReturnArityClause,
    ) -> Result<Vec<UnshiftedEntry<'a>>, SemanticError> {
        match arity_clause {
            mnode::ReturnArityClause::Unnamed(arity_literal) => {
                self.convert_return_arity_literal_to_context_extension(arity_literal, "_")
            }

            mnode::ReturnArityClause::Matchee(matchee_name, arity_literal) => self
                .convert_return_arity_literal_to_context_extension(
                    arity_literal,
                    &matchee_name.value,
                ),

            mnode::ReturnArityClause::Indices(index_names) => Ok(self
                .convert_return_arity_matchee_and_index_names_to_context_extension(
                    "_",
                    &index_names.idents,
                )),

            mnode::ReturnArityClause::MatcheeAndIndices(matchee_name, index_names) => Ok(self
                .convert_return_arity_matchee_and_index_names_to_context_extension(
                    &matchee_name.value,
                    &index_names.idents,
                )),
        }
    }

    fn convert_return_arity_literal_to_context_extension<'a>(
        &mut self,
        arity_literal: &'a mnode::ReturnArityLiteral,
        matchee_name: &'a str,
    ) -> Result<Vec<UnshiftedEntry<'a>>, SemanticError> {
        let arity = arity_literal.arity;

        let Some(arity_minus_one) = arity.checked_sub(1) else {
            return Err(SemanticError::ReturnArityIsZero(arity_literal.clone()));
        };

        let matchee_singleton = std::iter::once(self.get_deb_defining_entry(matchee_name));

        Ok((0..arity_minus_one)
            .map(|_| self.get_deb_defining_entry("_"))
            .chain(matchee_singleton)
            .collect())
    }

    fn convert_return_arity_matchee_and_index_names_to_context_extension<'a>(
        &mut self,
        matchee_name: &'a str,
        index_names: &'a mnode::CommaSeparatedIdentsOrUnderscores,
    ) -> Vec<UnshiftedEntry<'a>> {
        let mut out = vec![];
        self.push_index_names(index_names, &mut out);
        out.push(self.get_deb_defining_entry(matchee_name));
        out
    }

    fn push_index_names<'a>(
        &mut self,
        index_names: &'a mnode::CommaSeparatedIdentsOrUnderscores,
        out: &mut Vec<UnshiftedEntry<'a>>,
    ) {
        match index_names {
            mnode::CommaSeparatedIdentsOrUnderscores::One(ident_or_underscore) => {
                out.push(self.get_deb_defining_entry(ident_or_underscore.val()));
            }

            mnode::CommaSeparatedIdentsOrUnderscores::Snoc(rdc, rac) => {
                self.push_index_names(rdc, out);
                out.push(self.get_deb_defining_entry(rac.val()));
            }
        }
    }
}

impl MayConverter {
    pub(crate) fn convert_match_case_params_to_context_extension<'a>(
        &mut self,
        params: &'a mnode::OptParenthesizedCommaSeparatedIdentsOrUnderscores,
    ) -> Vec<UnshiftedEntry<'a>> {
        match params {
            mnode::OptParenthesizedCommaSeparatedIdentsOrUnderscores::None => vec![],

            mnode::OptParenthesizedCommaSeparatedIdentsOrUnderscores::Some(parenthesized) => {
                let mut out = vec![];
                self.push_index_names(&parenthesized.idents, &mut out);
                out
            }
        }
    }
}
