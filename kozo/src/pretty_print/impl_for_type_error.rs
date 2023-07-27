use super::*;

use crate::{
    syntax_tree::{ipist::rc_hashed, ipist_to_ast::*},
    typecheck::TypeError,
};

impl Display for PrettyPrint<'_, TypeError> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            TypeError::InvalidDeb { deb, tcon_len } => f
                .debug_struct("TypeError::InvalidDeb")
                .field("deb", &deb.value)
                .field("tcon_len", tcon_len)
                .finish(),

            TypeError::InvalidVconIndex(vcon) => {
                let mut converter = IpistToAstConverter::default();
                let vcon_ast = converter.convert_vcon(rc_hashed(vcon.clone()));
                f.debug_struct("TypeError::InvalidVconIndex")
                    .field("vcon", &vcon_ast.pretty_printed())
                    .finish()
            }

            TypeError::UnexpectedNonTypeExpression { expr, type_ } => {
                let mut converter = IpistToAstConverter::default();
                let expr_ast = converter.convert(expr.clone());
                f.debug_struct("TypeError::UnexpectedNonTypeExpression")
                    .field("expr", &expr_ast.pretty_printed())
                    .field("type_", &type_.raw().pretty_printed())
                    .finish()
            }

            TypeError::UniverseInconsistencyInIndDef {
                index_or_param_type,
                level,
                ind,
            } => {
                let mut converter = IpistToAstConverter::default();
                let index_or_param_type_ast = converter.convert(index_or_param_type.clone());
                let ind_ast = converter.convert_ind(rc_hashed(ind.clone()));
                f.debug_struct("TypeError::UniverseInconsistencyInIndDef")
                    .field(
                        "index_or_param_type",
                        &index_or_param_type_ast.pretty_printed(),
                    )
                    .field("level", &level)
                    .field("ind", &ind_ast.pretty_printed())
                    .finish()
            }

            TypeError::WrongNumberOfIndexArguments {
                def,
                expected,
                actual,
            } => {
                let mut converter = IpistToAstConverter::default();
                let def_ast = converter.convert_vcon_def(def.clone());
                f.debug_struct("TypeError::WrongNumberOfIndexArguments")
                    .field("def", &def_ast.pretty_printed())
                    .field("expected", expected)
                    .field("actual", actual)
                    .finish()
            }

            TypeError::NonInductiveMatcheeType { expr, type_ } => {
                let mut converter = IpistToAstConverter::default();
                let expr_ast = converter.convert(expr.clone());
                f.debug_struct("TypeError::NonInductiveMatcheeType")
                    .field("expr", &expr_ast.pretty_printed())
                    .field("type_", &type_.raw().pretty_printed())
                    .finish()
            }

            TypeError::WrongNumberOfMatchCases {
                match_,
                matchee_type_ind,
            } => {
                let mut converter = IpistToAstConverter::default();
                let match_ast = converter.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::WrongNumberOfMatchCases")
                    .field("match_", &match_ast.pretty_printed())
                    .field(
                        "matchee_type_ind",
                        &rc_hashed(matchee_type_ind.raw().clone()).pretty_printed(),
                    )
                    .finish()
            }

            TypeError::WrongMatchCaseArity {
                actual_node,
                expected,
                match_,
                match_case_index,
            } => {
                let mut converter = IpistToAstConverter::default();
                let match_ast = converter.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::WrongMatchCaseArity")
                    .field("actual_node", &actual_node.value)
                    .field("expected", expected)
                    .field("match_", &match_ast.pretty_printed())
                    .field("match_case_index", match_case_index)
                    .finish()
            }

            TypeError::IllegallyDismissedMatchCase {
                match_,
                match_case_index,
            } => {
                let mut converter = IpistToAstConverter::default();
                let match_ast = converter.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::IllegallyDismissedMatchCase")
                    .field("match_", &match_ast.pretty_printed())
                    .field("match_case_index", match_case_index)
                    .finish()
            }

            TypeError::TypeMismatch {
                expr,
                expected_type,
                actual_type,
            } => {
                let mut converter = IpistToAstConverter::default();
                let expr_ast = converter.convert(expr.clone());
                f.debug_struct("TypeError::TypeMismatch")
                    .field("expr", &expr_ast.pretty_printed())
                    .field("expected_type", &expected_type.raw().pretty_printed())
                    .field("actual_type", &actual_type.raw().pretty_printed())
                    .finish()
            }

            TypeError::CalleeTypeIsNotAForExpression { app, callee_type } => {
                let mut converter = IpistToAstConverter::default();
                let app_ast = converter.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::CalleeTypeIsNotAForExpression")
                    .field("app", &app_ast.pretty_printed())
                    .field("callee_type", &callee_type.raw().pretty_printed())
                    .finish()
            }

            TypeError::WrongNumberOfAppArguments {
                app,
                callee_type,
                expected,
                actual,
            } => {
                let mut converter = IpistToAstConverter::default();
                let app_ast = converter.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::WrongNumberOfAppArguments")
                    .field("app", &app_ast.pretty_printed())
                    .field(
                        "callee_type",
                        &rc_hashed(callee_type.raw().clone()).pretty_printed(),
                    )
                    .field("expected", expected)
                    .field("actual", actual)
                    .finish()
            }
        }
    }
}
