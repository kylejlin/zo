use super::*;

use crate::{
    syntax_tree::{
        ipist::rc_hashed,
        ipist_to_ast::*,
        token::{ByteIndex, Span},
    },
    typecheck::TypeError,
};

impl Display for PrettyPrint<'_, TypeError> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            TypeError::InvalidDeb { deb, tcon_len } => f
                .debug_struct("TypeError::InvalidDeb")
                .field("deb", &deb.value.with_location_appended(deb.span))
                .field("tcon_len", tcon_len)
                .finish(),

            TypeError::InvalidVconIndex(vcon) => {
                let mut converter = IpistToAstConverter::default();
                let vcon_ast = converter.convert_vcon(rc_hashed(vcon.clone()));
                f.debug_struct("TypeError::InvalidVconIndex")
                    .field(
                        "vcon",
                        &vcon_ast
                            .pretty_printed()
                            .with_location_appended(vcon.span()),
                    )
                    .finish()
            }

            TypeError::UnexpectedNonTypeExpression { expr, type_ } => {
                let mut converter = IpistToAstConverter::default();
                let expr_ast = converter.convert(expr.clone());
                f.debug_struct("TypeError::UnexpectedNonTypeExpression")
                    .field(
                        "expr",
                        &expr_ast
                            .pretty_printed()
                            .with_location_appended(expr.span()),
                    )
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
                        &index_or_param_type_ast
                            .pretty_printed()
                            .with_location_appended(index_or_param_type.span()),
                    )
                    .field("level", &level)
                    .field(
                        "ind",
                        &ind_ast.pretty_printed().with_location_appended(ind.span()),
                    )
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
                    .field(
                        "def",
                        &def_ast.pretty_printed().with_location_appended(def.span()),
                    )
                    .field("expected", expected)
                    .field("actual", actual)
                    .finish()
            }

            TypeError::NonInductiveMatcheeType { expr, type_ } => {
                let mut converter = IpistToAstConverter::default();
                let expr_ast = converter.convert(expr.clone());
                f.debug_struct("TypeError::NonInductiveMatcheeType")
                    .field(
                        "expr",
                        &expr_ast
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
                let mut converter = IpistToAstConverter::default();
                let match_ast = converter.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::WrongNumberOfMatchCases")
                    .field(
                        "match_",
                        &match_ast
                            .pretty_printed()
                            .with_location_appended(match_.span()),
                    )
                    .field(
                        "matchee_type_ind",
                        &rc_hashed(matchee_type_ind.raw().clone()).pretty_printed(),
                    )
                    .finish()
            }

            TypeError::WrongMatchReturnTypeArity {
                match_,
                matchee_type_args,
            } => {
                let mut converter = IpistToAstConverter::default();
                let match_ast = converter.convert_match(rc_hashed(match_.clone()));
                let matchee_type_args: Vec<_> = matchee_type_args
                    .iter()
                    .map(|arg| arg.raw().pretty_printed())
                    .collect();
                f.debug_struct("TypeError::WrongMatchReturnTypeArity")
                    .field(
                        "match_",
                        &match_ast
                            .pretty_printed()
                            .with_location_appended(match_.span()),
                    )
                    .field("matchee_type_args", &matchee_type_args)
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
                    .field(
                        "actual_node",
                        &actual_node.value.with_location_appended(actual_node.span),
                    )
                    .field("expected", expected)
                    .field(
                        "match_",
                        &match_ast
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
                let mut converter = IpistToAstConverter::default();
                let expr_ast = converter.convert(expr.clone());
                f.debug_struct("TypeError::TypeMismatch")
                    .field(
                        "expr",
                        &expr_ast
                            .pretty_printed()
                            .with_location_appended(expr.span()),
                    )
                    .field("expected_type", &expected_type.raw().pretty_printed())
                    .field("actual_type", &actual_type.raw().pretty_printed())
                    .finish()
            }

            TypeError::CalleeTypeIsNotAForExpression { app, callee_type } => {
                let mut converter = IpistToAstConverter::default();
                let app_ast = converter.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::CalleeTypeIsNotAForExpression")
                    .field(
                        "app",
                        &app_ast.pretty_printed().with_location_appended(app.span()),
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
                let mut converter = IpistToAstConverter::default();
                let app_ast = converter.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::WrongNumberOfAppArguments")
                    .field(
                        "app",
                        &app_ast.pretty_printed().with_location_appended(app.span()),
                    )
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

struct AppendLocation<T> {
    val: T,
    start: ByteIndex,
    end: ByteIndex,
}

impl<T> Debug for AppendLocation<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl<T> Display for AppendLocation<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let val = &self.val;
        let start = self.start;
        let end = self.end;
        write!(f, "{val:?}@({start:?}..{end:?})")
    }
}

trait WithLocationAppended: Sized {
    fn with_location_appended(self, span: Span) -> AppendLocation<Self>;
}

impl<T> WithLocationAppended for T {
    fn with_location_appended(self, (start, end): Span) -> AppendLocation<Self> {
        AppendLocation {
            val: self,
            start,
            end,
        }
    }
}
