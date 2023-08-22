use super::*;

use crate::{
    syntax_tree::{
        minimal_ast,
        remove_ast_aux_data::*,
        spanned_ast::{rc_hashed, SpanAuxData},
    },
    typecheck::TypeError,
};

impl Display for PrettyPrint<'_, TypeError<SpanAuxData>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            TypeError::InvalidDeb { deb, tcon_len } => {
                let deb_minimal = minimal_ast::DebNode {
                    deb: deb.deb,
                    aux_data: (),
                };
                f.debug_struct("TypeError::InvalidDeb")
                    .field(
                        "deb",
                        &deb_minimal
                            .pretty_printed()
                            .with_location_appended(deb.span()),
                    )
                    .field("tcon_len", tcon_len)
                    .finish()
            }

            TypeError::InvalidVconIndex(vcon) => {
                let mut converter = AuxDataRemover::default();
                let vcon_minimal = converter.convert_vcon(rc_hashed(vcon.clone()));
                f.debug_struct("TypeError::InvalidVconIndex")
                    .field(
                        "vcon",
                        &vcon_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(vcon.span()),
                    )
                    .finish()
            }

            TypeError::UnexpectedNonTypeExpression { expr, type_ } => {
                let mut converter = AuxDataRemover::default();
                let expr_minimal = converter.convert(expr.clone());
                f.debug_struct("TypeError::UnexpectedNonTypeExpression")
                    .field(
                        "expr",
                        &expr_minimal
                            .pretty_printed()
                            .with_location_appended(expr.span()),
                    )
                    .field("type_", &type_.raw().pretty_printed())
                    .finish()
            }

            TypeError::UniverseInconsistencyInIndDef {
                index_or_param_type,
                universe,
                ind,
            } => {
                let mut converter = AuxDataRemover::default();
                let index_or_param_type_minimal = converter.convert(index_or_param_type.clone());
                let ind_minimal = converter.convert_ind(rc_hashed(ind.clone()));
                f.debug_struct("TypeError::UniverseInconsistencyInIndDef")
                    .field(
                        "index_or_param_type",
                        &index_or_param_type_minimal
                            .pretty_printed()
                            .with_location_appended(index_or_param_type.span()),
                    )
                    .field("universe", &universe)
                    .field(
                        "ind",
                        &ind_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(ind.span()),
                    )
                    .finish()
            }

            TypeError::WrongNumberOfIndexArguments {
                def,
                expected,
                actual,
            } => {
                let mut converter = AuxDataRemover::default();
                let def_minimal = converter.convert_vcon_def(def.clone());
                f.debug_struct("TypeError::WrongNumberOfIndexArguments")
                    .field(
                        "def",
                        &def_minimal
                            .pretty_printed()
                            .with_location_appended(def.span()),
                    )
                    .field("expected", expected)
                    .field("actual", actual)
                    .finish()
            }

            TypeError::NonInductiveMatcheeType { expr, type_ } => {
                let mut converter = AuxDataRemover::default();
                let expr_minimal = converter.convert(expr.clone());
                f.debug_struct("TypeError::NonInductiveMatcheeType")
                    .field(
                        "expr",
                        &expr_minimal
                            .pretty_printed()
                            .with_location_appended(expr.span()),
                    )
                    .field("type_", &type_.raw().pretty_printed())
                    .finish()
            }

            TypeError::WrongNumberOfMatchCases {
                match_,
                matchee_type_ind,
            } => {
                let mut converter = AuxDataRemover::default();
                let match_minimal = converter.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::WrongNumberOfMatchCases")
                    .field(
                        "match_",
                        &match_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(match_.span()),
                    )
                    .field("matchee_type_ind", &matchee_type_ind.raw().pretty_printed())
                    .finish()
            }

            TypeError::WrongMatchReturnTypeArity {
                match_,
                matchee_type_args,
            } => {
                let mut converter = AuxDataRemover::default();
                let match_minimal = converter.convert_match(rc_hashed(match_.clone()));
                let matchee_type_args: Vec<_> = matchee_type_args
                    .iter()
                    .map(|arg| arg.raw().pretty_printed())
                    .collect();
                f.debug_struct("TypeError::WrongMatchReturnTypeArity")
                    .field(
                        "match_",
                        &match_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(match_.span()),
                    )
                    .field("matchee_type_args", &matchee_type_args)
                    .finish()
            }

            TypeError::WrongMatchCaseArity {
                stated_arity,
                expected,
                match_,
                match_case_index,
            } => {
                let mut converter = AuxDataRemover::default();
                let match_minimal = converter.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::WrongMatchCaseArity")
                    .field("stated_arity", &stated_arity)
                    .field("expected", expected)
                    .field(
                        "match_",
                        &match_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(match_.span()),
                    )
                    .field("match_case_index", match_case_index)
                    .finish()
            }

            TypeError::TypeMismatch {
                expr,
                expected_type,
                actual_type,
            } => {
                let mut converter = AuxDataRemover::default();
                let expr_minimal = converter.convert(expr.clone());
                f.debug_struct("TypeError::TypeMismatch")
                    .field(
                        "expr",
                        &expr_minimal
                            .pretty_printed()
                            .with_location_appended(expr.span()),
                    )
                    .field("expected_type", &expected_type.raw().pretty_printed())
                    .field("actual_type", &actual_type.raw().pretty_printed())
                    .finish()
            }

            TypeError::CalleeTypeIsNotAForExpression { app, callee_type } => {
                let mut converter = AuxDataRemover::default();
                let app_minimal = converter.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::CalleeTypeIsNotAForExpression")
                    .field(
                        "app",
                        &app_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(app.span()),
                    )
                    .field("callee_type", &callee_type.raw().pretty_printed())
                    .finish()
            }

            TypeError::WrongNumberOfAppArguments {
                app,
                callee_type,
                expected,
                actual,
            } => {
                let mut converter = AuxDataRemover::default();
                let app_minimal = converter.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::WrongNumberOfAppArguments")
                    .field(
                        "app",
                        &app_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(app.span()),
                    )
                    .field("callee_type", &callee_type.raw().pretty_printed())
                    .field("expected", expected)
                    .field("actual", actual)
                    .finish()
            }

            TypeError::FunHasZeroParams { fun } => {
                let mut converter = AuxDataRemover::default();
                let fun_minimal = converter.convert(fun.clone().into());
                f.debug_struct("TypeError::FunHasZeroParams")
                    .field(
                        "fun",
                        &fun_minimal
                            .pretty_printed()
                            .with_location_appended(fun.span()),
                    )
                    .finish()
            }

            TypeError::AppHasZeroArgs { app } => {
                let mut converter = AuxDataRemover::default();
                let app_minimal = converter.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::AppHasZeroArgs")
                    .field(
                        "app",
                        &app_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(app.span()),
                    )
                    .finish()
            }

            TypeError::ForHasZeroParams { for_ } => {
                let mut converter = AuxDataRemover::default();
                let for_minimal = converter.convert(for_.clone().into());
                f.debug_struct("TypeError::ForHasZeroParams")
                    .field(
                        "for_",
                        &for_minimal
                            .pretty_printed()
                            .with_location_appended(for_.span()),
                    )
                    .finish()
            }

            TypeError::IllegalRecursiveCall {
                app,
                callee_deb_definition_src,
                required_decreasing_arg_index,
                required_strict_superstruct,
            } => {
                let mut converter = AuxDataRemover::default();
                let app_minimal = converter.convert_app(rc_hashed(app.clone()));
                let callee_deb_definition_src_minimal =
                    converter.convert(callee_deb_definition_src.clone().into());
                f.debug_struct("TypeError::IllegalRecursiveCall")
                    .field(
                        "app",
                        &app_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(app.span()),
                    )
                    .field(
                        "callee_deb_definition_src",
                        &callee_deb_definition_src_minimal
                            .pretty_printed()
                            .with_location_appended(callee_deb_definition_src.span()),
                    )
                    .field(
                        "required_decreasing_arg_index",
                        required_decreasing_arg_index,
                    )
                    .field("required_strict_superstruct", &required_strict_superstruct)
                    .finish()
            }

            TypeError::RecursiveFunParamInNonCalleePosition {
                deb,
                definition_src,
            } => {
                let mut converter = AuxDataRemover::default();
                let deb_minimal = minimal_ast::DebNode {
                    deb: deb.deb,
                    aux_data: (),
                };
                let definition_src_minimal = converter.convert(definition_src.clone().into());
                f.debug_struct("TypeError::RecursiveFunParamInNonCalleePosition")
                    .field(
                        "deb",
                        &deb_minimal
                            .pretty_printed()
                            .with_location_appended(deb.span()),
                    )
                    .field(
                        "definition_src",
                        &definition_src_minimal
                            .pretty_printed()
                            .with_location_appended(definition_src.span()),
                    )
                    .finish()
            }

            TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam {
                deb,
                definition_src,
            } => {
                let mut converter = AuxDataRemover::default();
                let deb_minimal = minimal_ast::DebNode {
                    deb: deb.deb,
                    aux_data: (),
                };
                let definition_src_minimal = converter.convert(definition_src.clone().into());
                f.debug_struct("TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam")
                    .field(
                        "deb",
                        &deb_minimal
                            .pretty_printed()
                            .with_location_appended(deb.span()),
                    )
                    .field(
                        "definition_src",
                        &definition_src_minimal
                            .pretty_printed()
                            .with_location_appended(definition_src.span()),
                    )
                    .finish()
            }

            TypeError::DecreasingArgIndexTooBig { fun } => {
                let mut converter = AuxDataRemover::default();
                let fun_minimal = converter.convert(fun.clone().into());
                f.debug_struct("TypeError::DecreasingArgIndexTooBig")
                    .field(
                        "fun",
                        &fun_minimal
                            .pretty_printed()
                            .with_location_appended(fun.span()),
                    )
                    .finish()
            }

            TypeError::VconDefParamTypeFailsStrictPositivityCondition {
                def,
                param_type_index,
                normalized_param_type,
                path_from_param_type_to_problematic_deb,
            } => {
                let mut converter = AuxDataRemover::default();
                let def_minimal = converter.convert_vcon_def(def.clone());
                f.debug_struct("TypeError::VconDefParamTypeFailsStrictPositivityCondition")
                    .field(
                        "def",
                        &def_minimal
                            .pretty_printed()
                            .with_location_appended(def.span()),
                    )
                    .field("param_type_index", param_type_index)
                    .field(
                        "normalized_param_type",
                        &normalized_param_type.raw().pretty_printed(),
                    )
                    .field(
                        "path_from_param_type_to_problematic_deb",
                        path_from_param_type_to_problematic_deb,
                    )
                    .finish()
            }

            TypeError::RecursiveIndParamAppearsInVconDefIndexArg {
                def,
                index_arg_index,
                normalized_index_arg,
                path_from_index_arg_to_problematic_deb,
            } => {
                let mut converter = AuxDataRemover::default();
                let def_minimal = converter.convert_vcon_def(def.clone());
                f.debug_struct("TypeError::RecursiveIndParamAppearsInVconDefIndexArg")
                    .field(
                        "def",
                        &def_minimal
                            .pretty_printed()
                            .with_location_appended(def.span()),
                    )
                    .field("index_arg_index", index_arg_index)
                    .field(
                        "normalized_index_arg",
                        &normalized_index_arg.raw().pretty_printed(),
                    )
                    .field(
                        "path_from_index_arg_to_problematic_deb",
                        path_from_index_arg_to_problematic_deb,
                    )
                    .finish()
            }

            TypeError::MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable {
                match_,
                matchee_type_type,
                match_return_type_type,
            } => {
                let mut converter = AuxDataRemover::default();
                let match_minimal = converter.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable")
                    .field(
                        "match_",
                        &match_minimal
                            .hashee
                            .pretty_printed()
                            .with_location_appended(match_.span()),
                    )
                    .field("matchee_type_type", &matchee_type_type.pretty_printed())
                    .field(
                        "match_return_type_type",
                        &match_return_type_type.pretty_printed(),
                    )
                    .finish()
            }
        }
    }
}

// TODO: Rename `let mut converter` to `let mut remover`
// (all over the crate, not just in this file).
