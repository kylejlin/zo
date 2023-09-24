use super::*;

use crate::check_erasability::ErasabilityError;

impl Display for PrettyPrint<'_, ErasabilityError> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            ErasabilityError::MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable {
                match_,
                matchee_type_type,
                match_return_type_type,
            } => f
                .debug_struct(
                    "ErasabilityError::MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable",
                )
                .field("match_", &match_.pretty_printed())
                .field("matchee_type_type", &matchee_type_type.pretty_printed())
                .field(
                    "match_return_type_type",
                    &match_return_type_type.pretty_printed(),
                )
                .finish(),
        }
    }
}
