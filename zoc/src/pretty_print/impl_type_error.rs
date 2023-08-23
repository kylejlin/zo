use super::*;

use crate::{
    syntax_tree::{
        ast::prelude::{minimal_ast::UnitAuxData, spanned_ast::SpanAuxData, *},
        remove_ast_aux_data::*,
    },
    typecheck::TypeError,
};

impl<A> Display for PrettyPrint<'_, TypeError<A>>
where
    A: AuxDataFamilyWhoseAstFamilyImplsGetOptSpan,
    ast::Expr<A>: GetOptSpan,
    ast::Ind<A>: GetOptSpan,
    ast::VconDef<A>: GetOptSpan,
    ast::Vcon<A>: GetOptSpan,
    ast::Match<A>: GetOptSpan,
    ast::MatchCase<A>: GetOptSpan,
    ast::Fun<A>: GetOptSpan,
    ast::App<A>: GetOptSpan,
    ast::For<A>: GetOptSpan,
    ast::DebNode<A>: GetOptSpan,
    ast::UniverseNode<A>: GetOptSpan,
{
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
                            .with_opt_location_appended(deb.opt_span()),
                    )
                    .field("tcon_len", tcon_len)
                    .finish()
            }

            TypeError::InvalidVconIndex(vcon) => {
                let mut remover = AuxDataRemover::default();
                let vcon_minimal = remover.convert_vcon(rc_hashed(vcon.clone()));
                f.debug_struct("TypeError::InvalidVconIndex")
                    .field(
                        "vcon",
                        &vcon_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(vcon.opt_span()),
                    )
                    .finish()
            }

            TypeError::UnexpectedNonTypeExpression { expr, type_ } => {
                let mut remover = AuxDataRemover::default();
                let expr_minimal = remover.convert(expr.clone());
                f.debug_struct("TypeError::UnexpectedNonTypeExpression")
                    .field(
                        "expr",
                        &expr_minimal
                            .pretty_printed()
                            .with_opt_location_appended(expr.opt_span()),
                    )
                    .field("type_", &type_.raw().pretty_printed())
                    .finish()
            }

            TypeError::UniverseInconsistencyInIndDef {
                index_or_param_type,
                universe,
                ind,
            } => {
                let mut remover = AuxDataRemover::default();
                let index_or_param_type_minimal = remover.convert(index_or_param_type.clone());
                let ind_minimal = remover.convert_ind(rc_hashed(ind.clone()));
                f.debug_struct("TypeError::UniverseInconsistencyInIndDef")
                    .field(
                        "index_or_param_type",
                        &index_or_param_type_minimal
                            .pretty_printed()
                            .with_opt_location_appended(index_or_param_type.opt_span()),
                    )
                    .field("universe", &universe)
                    .field(
                        "ind",
                        &ind_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(ind.opt_span()),
                    )
                    .finish()
            }

            TypeError::WrongNumberOfIndexArguments {
                def,
                expected,
                actual,
            } => {
                let mut remover = AuxDataRemover::default();
                let def_minimal = remover.convert_vcon_def(def.clone());
                f.debug_struct("TypeError::WrongNumberOfIndexArguments")
                    .field(
                        "def",
                        &def_minimal
                            .pretty_printed()
                            .with_opt_location_appended(def.opt_span()),
                    )
                    .field("expected", expected)
                    .field("actual", actual)
                    .finish()
            }

            TypeError::NonInductiveMatcheeType { expr, type_ } => {
                let mut remover = AuxDataRemover::default();
                let expr_minimal = remover.convert(expr.clone());
                f.debug_struct("TypeError::NonInductiveMatcheeType")
                    .field(
                        "expr",
                        &expr_minimal
                            .pretty_printed()
                            .with_opt_location_appended(expr.opt_span()),
                    )
                    .field("type_", &type_.raw().pretty_printed())
                    .finish()
            }

            TypeError::WrongNumberOfMatchCases {
                match_,
                matchee_type_ind,
            } => {
                let mut remover = AuxDataRemover::default();
                let match_minimal = remover.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::WrongNumberOfMatchCases")
                    .field(
                        "match_",
                        &match_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(match_.opt_span()),
                    )
                    .field("matchee_type_ind", &matchee_type_ind.raw().pretty_printed())
                    .finish()
            }

            TypeError::WrongMatchReturnTypeArity {
                match_,
                matchee_type_args,
            } => {
                let mut remover = AuxDataRemover::default();
                let match_minimal = remover.convert_match(rc_hashed(match_.clone()));
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
                            .with_opt_location_appended(match_.opt_span()),
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
                let mut remover = AuxDataRemover::default();
                let match_minimal = remover.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::WrongMatchCaseArity")
                    .field("stated_arity", &stated_arity)
                    .field("expected", expected)
                    .field(
                        "match_",
                        &match_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(match_.opt_span()),
                    )
                    .field("match_case_index", match_case_index)
                    .finish()
            }

            TypeError::TypeMismatch {
                expr,
                expected_type,
                actual_type,
            } => {
                let mut remover = AuxDataRemover::default();
                let expr_minimal = remover.convert(expr.clone());
                f.debug_struct("TypeError::TypeMismatch")
                    .field(
                        "expr",
                        &expr_minimal
                            .pretty_printed()
                            .with_opt_location_appended(expr.opt_span()),
                    )
                    .field("expected_type", &expected_type.raw().pretty_printed())
                    .field("actual_type", &actual_type.raw().pretty_printed())
                    .finish()
            }

            TypeError::CalleeTypeIsNotAForExpression { app, callee_type } => {
                let mut remover = AuxDataRemover::default();
                let app_minimal = remover.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::CalleeTypeIsNotAForExpression")
                    .field(
                        "app",
                        &app_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(app.opt_span()),
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
                let mut remover = AuxDataRemover::default();
                let app_minimal = remover.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::WrongNumberOfAppArguments")
                    .field(
                        "app",
                        &app_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(app.opt_span()),
                    )
                    .field("callee_type", &callee_type.raw().pretty_printed())
                    .field("expected", expected)
                    .field("actual", actual)
                    .finish()
            }

            TypeError::FunHasZeroParams { fun } => {
                let mut remover = AuxDataRemover::default();
                let fun_minimal = remover.convert(fun.clone().into());
                f.debug_struct("TypeError::FunHasZeroParams")
                    .field(
                        "fun",
                        &fun_minimal
                            .pretty_printed()
                            .with_opt_location_appended(fun.opt_span()),
                    )
                    .finish()
            }

            TypeError::AppHasZeroArgs { app } => {
                let mut remover = AuxDataRemover::default();
                let app_minimal = remover.convert_app(rc_hashed(app.clone()));
                f.debug_struct("TypeError::AppHasZeroArgs")
                    .field(
                        "app",
                        &app_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(app.opt_span()),
                    )
                    .finish()
            }

            TypeError::ForHasZeroParams { for_ } => {
                let mut remover = AuxDataRemover::default();
                let for_minimal = remover.convert(for_.clone().into());
                f.debug_struct("TypeError::ForHasZeroParams")
                    .field(
                        "for_",
                        &for_minimal
                            .pretty_printed()
                            .with_opt_location_appended(for_.opt_span()),
                    )
                    .finish()
            }

            TypeError::IllegalRecursiveCall {
                app,
                callee_deb_definition_src,
                required_decreasing_arg_index,
                required_strict_superstruct,
            } => {
                let mut remover = AuxDataRemover::default();
                let app_minimal = remover.convert_app(rc_hashed(app.clone()));
                let callee_deb_definition_src_minimal =
                    remover.convert(callee_deb_definition_src.clone().into());
                f.debug_struct("TypeError::IllegalRecursiveCall")
                    .field(
                        "app",
                        &app_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(app.opt_span()),
                    )
                    .field(
                        "callee_deb_definition_src",
                        &callee_deb_definition_src_minimal
                            .pretty_printed()
                            .with_opt_location_appended(callee_deb_definition_src.opt_span()),
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
                let mut remover = AuxDataRemover::default();
                let deb_minimal = minimal_ast::DebNode {
                    deb: deb.deb,
                    aux_data: (),
                };
                let definition_src_minimal = remover.convert(definition_src.clone().into());
                f.debug_struct("TypeError::RecursiveFunParamInNonCalleePosition")
                    .field(
                        "deb",
                        &deb_minimal
                            .pretty_printed()
                            .with_opt_location_appended(deb.opt_span()),
                    )
                    .field(
                        "definition_src",
                        &definition_src_minimal
                            .pretty_printed()
                            .with_opt_location_appended(definition_src.opt_span()),
                    )
                    .finish()
            }

            TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam {
                deb,
                definition_src,
            } => {
                let mut remover = AuxDataRemover::default();
                let deb_minimal = minimal_ast::DebNode {
                    deb: deb.deb,
                    aux_data: (),
                };
                let definition_src_minimal = remover.convert(definition_src.clone().into());
                f.debug_struct("TypeError::DeclaredFunNonrecursiveButUsedRecursiveFunParam")
                    .field(
                        "deb",
                        &deb_minimal
                            .pretty_printed()
                            .with_opt_location_appended(deb.opt_span()),
                    )
                    .field(
                        "definition_src",
                        &definition_src_minimal
                            .pretty_printed()
                            .with_opt_location_appended(definition_src.opt_span()),
                    )
                    .finish()
            }

            TypeError::DecreasingArgIndexTooBig { fun } => {
                let mut remover = AuxDataRemover::default();
                let fun_minimal = remover.convert(fun.clone().into());
                f.debug_struct("TypeError::DecreasingArgIndexTooBig")
                    .field(
                        "fun",
                        &fun_minimal
                            .pretty_printed()
                            .with_opt_location_appended(fun.opt_span()),
                    )
                    .finish()
            }

            TypeError::VconDefParamTypeFailsStrictPositivityCondition {
                def,
                param_type_index,
                normalized_param_type,
                path_from_param_type_to_problematic_deb,
            } => {
                let mut remover = AuxDataRemover::default();
                let def_minimal = remover.convert_vcon_def(def.clone());
                f.debug_struct("TypeError::VconDefParamTypeFailsStrictPositivityCondition")
                    .field(
                        "def",
                        &def_minimal
                            .pretty_printed()
                            .with_opt_location_appended(def.opt_span()),
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
                let mut remover = AuxDataRemover::default();
                let def_minimal = remover.convert_vcon_def(def.clone());
                f.debug_struct("TypeError::RecursiveIndParamAppearsInVconDefIndexArg")
                    .field(
                        "def",
                        &def_minimal
                            .pretty_printed()
                            .with_opt_location_appended(def.opt_span()),
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
                let mut remover = AuxDataRemover::default();
                let match_minimal = remover.convert_match(rc_hashed(match_.clone()));
                f.debug_struct("TypeError::MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable")
                    .field(
                        "match_",
                        &match_minimal
                            .hashee
                            .pretty_printed()
                            .with_opt_location_appended(match_.opt_span()),
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

/// A marker trait for aux data families
/// whose corresponding AST families
/// implement `GetOptSpan`.
trait AuxDataFamilyWhoseAstFamilyImplsGetOptSpan: AuxDataFamily
where
    ast::Expr<Self>: GetOptSpan,
    ast::Ind<Self>: GetOptSpan,
    ast::VconDef<Self>: GetOptSpan,
    ast::Vcon<Self>: GetOptSpan,
    ast::Match<Self>: GetOptSpan,
    ast::MatchCase<Self>: GetOptSpan,
    ast::Fun<Self>: GetOptSpan,
    ast::App<Self>: GetOptSpan,
    ast::For<Self>: GetOptSpan,
    ast::DebNode<Self>: GetOptSpan,
    ast::UniverseNode<Self>: GetOptSpan,
{
}

impl AuxDataFamilyWhoseAstFamilyImplsGetOptSpan for SpanAuxData {}

impl GetOptSpan for spanned_ast::Expr {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::Ind {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::VconDef {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::Vcon {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::Match {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::MatchCase {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::Fun {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::App {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::For {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::DebNode {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}
impl GetOptSpan for spanned_ast::UniverseNode {
    fn opt_span(&self) -> Option<Span> {
        Some(self.span())
    }
}

impl AuxDataFamilyWhoseAstFamilyImplsGetOptSpan for UnitAuxData {}

impl GetOptSpan for minimal_ast::Expr {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::Ind {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::VconDef {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::Vcon {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::Match {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::MatchCase {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::Fun {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::App {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::For {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::DebNode {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}
impl GetOptSpan for minimal_ast::UniverseNode {
    fn opt_span(&self) -> Option<Span> {
        None
    }
}

trait GetOptSpan {
    fn opt_span(&self) -> Option<Span>;
}

pub struct AppendOptLocation<T> {
    pub val: T,
    pub span: Option<Span>,
}

impl<T> Debug for AppendOptLocation<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self, f)
    }
}

impl<T> Display for AppendOptLocation<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(span) = self.span {
            Display::fmt(
                &AppendLocation {
                    val: &self.val,
                    span,
                },
                f,
            )
        } else {
            self.val.fmt(f)
        }
    }
}

pub trait WithOptLocationAppended: Sized {
    fn with_opt_location_appended(self, span: Option<Span>) -> AppendOptLocation<Self>;
}

impl<T> WithOptLocationAppended for T {
    fn with_opt_location_appended(self, span: Option<Span>) -> AppendOptLocation<Self> {
        AppendOptLocation { val: self, span }
    }
}
