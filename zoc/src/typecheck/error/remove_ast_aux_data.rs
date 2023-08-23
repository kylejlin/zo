use super::*;

impl<A: AuxDataFamily> TypeError<A> {
    pub fn remove_ast_aux_data(self, remover: &mut AuxDataRemover) -> TypeError<UnitAuxData> {
        match self {
            TypeError::InvalidDeb { deb, tcon_len } => TypeError::InvalidDeb {
                deb: remover.convert_deb_node(&deb).hashee.clone(),
                tcon_len,
            },

            TypeError::InvalidVconIndex(vcon) => {
                TypeError::InvalidVconIndex(remover.convert_vcon(rc_hashed(vcon)).hashee.clone())
            }

            TypeError::UnexpectedNonTypeExpression { expr, type_ } => {
                TypeError::UnexpectedNonTypeExpression {
                    expr: remover.convert(expr).clone(),
                    type_,
                }
            }

            TypeError::UniverseInconsistencyInIndDef {
                index_or_param_type,
                universe,
                ind,
            } => TypeError::UniverseInconsistencyInIndDef {
                index_or_param_type: remover.convert(index_or_param_type).clone(),
                universe,
                ind: remover.convert_ind(rc_hashed(ind)).hashee.clone(),
            },

            TypeError::WrongNumberOfIndexArguments {
                def,
                expected,
                actual,
            } => TypeError::WrongNumberOfIndexArguments {
                def: remover.convert_vcon_def(def).clone(),
                expected,
                actual,
            },

            TypeError::NonInductiveMatcheeType { expr, type_ } => {
                TypeError::NonInductiveMatcheeType {
                    expr: remover.convert(expr).clone(),
                    type_,
                }
            }

            TypeError::WrongNumberOfMatchCases {
                match_,
                matchee_type_ind,
            } => TypeError::WrongNumberOfMatchCases {
                match_: remover.convert_match(rc_hashed(match_)).hashee.clone(),
                matchee_type_ind,
            },

            TypeError::WrongMatchReturnTypeArity {
                match_,
                matchee_type_args,
            } => TypeError::WrongMatchReturnTypeArity {
                match_: remover.convert_match(rc_hashed(match_)).hashee.clone(),
                matchee_type_args,
            },

            TypeError::WrongMatchCaseArity {
                stated_arity,
                expected,
                match_,
                match_case_index,
            } => TypeError::WrongMatchCaseArity {
                stated_arity,
                expected,
                match_: remover.convert_match(rc_hashed(match_)).hashee.clone(),
                match_case_index,
            },

            TypeError::TypeMismatch {
                expr,
                expected_type,
                actual_type,
            } => TypeError::TypeMismatch {
                expr: remover.convert(expr).clone(),
                expected_type,
                actual_type,
            },

            TypeError::CalleeTypeIsNotAForExpression { app, callee_type } => {
                TypeError::CalleeTypeIsNotAForExpression {
                    app: remover.convert_app(rc_hashed(app)).hashee.clone(),
                    callee_type,
                }
            }

            TypeError::WrongNumberOfAppArguments {
                app,
                callee_type,
                expected,
                actual,
            } => TypeError::WrongNumberOfAppArguments {
                app: remover.convert_app(rc_hashed(app)).hashee.clone(),
                callee_type,
                expected,
                actual,
            },

            TypeError::FunHasZeroParams { fun } => TypeError::FunHasZeroParams {
                fun: remover.convert_fun(rc_hashed(fun)).hashee.clone(),
            },

            TypeError::AppHasZeroArgs { app } => TypeError::AppHasZeroArgs {
                app: remover.convert_app(rc_hashed(app)).hashee.clone(),
            },

            TypeError::ForHasZeroParams { for_ } => TypeError::ForHasZeroParams {
                for_: remover.convert_for(rc_hashed(for_)).hashee.clone(),
            },

            TypeError::IllegalRecursiveCall {
                app,
                callee_deb_definition_src,
                required_decreasing_arg_index,
                required_strict_superstruct,
            } => TypeError::IllegalRecursiveCall {
                app: remover.convert_app(rc_hashed(app)).hashee.clone(),
                callee_deb_definition_src: remover
                    .convert_fun(rc_hashed(callee_deb_definition_src))
                    .hashee
                    .clone(),
                required_decreasing_arg_index,
                required_strict_superstruct,
            },

            TypeError::RecursiveFunParamInNonCalleePosition {
                deb,
                definition_src,
            } => TypeError::RecursiveFunParamInNonCalleePosition {
                deb: remover.convert_deb_node(&deb).hashee.clone(),
                definition_src: remover
                    .convert_fun(rc_hashed(definition_src))
                    .hashee
                    .clone(),
            },

            TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam {
                deb,
                definition_src,
            } => TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam {
                deb: remover.convert_deb_node(&deb).hashee.clone(),
                definition_src: remover
                    .convert_fun(rc_hashed(definition_src))
                    .hashee
                    .clone(),
            },

            TypeError::DecreasingArgIndexTooBig { fun } => TypeError::DecreasingArgIndexTooBig {
                fun: remover.convert_fun(rc_hashed(fun)).hashee.clone(),
            },

            TypeError::VconDefParamTypeFailsStrictPositivityCondition {
                def,
                param_type_index,
                normalized_param_type,
                path_from_param_type_to_problematic_deb,
            } => TypeError::VconDefParamTypeFailsStrictPositivityCondition {
                def: remover.convert_vcon_def(def).clone(),
                param_type_index,
                normalized_param_type,
                path_from_param_type_to_problematic_deb,
            },

            TypeError::RecursiveIndParamAppearsInVconDefIndexArg {
                def,
                index_arg_index,
                normalized_index_arg,
                path_from_index_arg_to_problematic_deb,
            } => TypeError::RecursiveIndParamAppearsInVconDefIndexArg {
                def: remover.convert_vcon_def(def).clone(),
                index_arg_index,
                normalized_index_arg,
                path_from_index_arg_to_problematic_deb,
            },

            TypeError::MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable {
                match_,
                matchee_type_type,
                match_return_type_type,
            } => TypeError::MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable {
                match_: remover.convert_match(rc_hashed(match_)).hashee.clone(),
                matchee_type_type,
                match_return_type_type,
            },
        }
    }
}
