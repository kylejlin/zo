use super::*;

use crate::pretty_print::PrettyPrinted;

use std::fmt::{Debug, Result as FmtResult};

/// We parameterize `TypeError`s over AST families
/// (i.e., using the `A` type parameter)
/// so that you can typecheck an AST in any AST family.
/// If the typechecking results in an error,
/// the error will be in the same AST family
/// as the family of the input AST.
///
/// So, for example, if you typecheck an AST with
/// span information (i.e., a node in the `spanned_ast` family),
/// then the error will also include span information.
/// On the other hand, if you typecheck an AST with
/// no auxiliary information whatsoever (i.e., a node in the `minimal_ast` family),
/// then the error will also have no auxiliary information.
#[derive(Clone)]
pub enum TypeError<A: AuxDataFamily> {
    InvalidDeb {
        deb: ast::DebNode<A>,
        tcon_len: usize,
    },
    InvalidVconIndex(ast::Vcon<A>),
    UnexpectedNonTypeExpression {
        expr: ast::Expr<A>,
        type_: NormalForm,
    },
    UniverseInconsistencyInIndDef {
        index_or_param_type: ast::Expr<A>,
        universe: Universe,
        ind: ast::Ind<A>,
    },
    WrongNumberOfIndexArguments {
        def: ast::VconDef<A>,
        expected: usize,
        actual: usize,
    },
    NonInductiveMatcheeType {
        expr: ast::Expr<A>,
        type_: NormalForm,
    },
    WrongNumberOfMatchCases {
        match_: ast::Match<A>,
        matchee_type_ind: Normalized<minimal_ast::Ind>,
    },
    WrongMatchReturnTypeArity {
        match_: ast::Match<A>,
        matchee_type_args: Vec<NormalForm>,
    },
    WrongMatchCaseArity {
        stated_arity: usize,
        expected: usize,
        match_: ast::Match<A>,
        match_case_index: usize,
    },
    TypeMismatch {
        expr: ast::Expr<A>,
        expected_type: NormalForm,
        actual_type: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: ast::App<A>,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: ast::App<A>,
        callee_type: Normalized<minimal_ast::For>,
        expected: usize,
        actual: usize,
    },
    FunHasZeroParams {
        fun: ast::Fun<A>,
    },
    AppHasZeroArgs {
        app: ast::App<A>,
    },
    ForHasZeroParams {
        for_: ast::For<A>,
    },

    IllegalRecursiveCall {
        app: ast::App<A>,
        callee_deb_definition_src: ast::Fun<A>,
        required_decreasing_arg_index: usize,
        required_strict_superstruct: Deb,
    },
    RecursiveFunParamInNonCalleePosition {
        deb: ast::DebNode<A>,
        definition_src: ast::Fun<A>,
    },
    DeclaredFunNonrecursiveButUsedRecursiveFunParam {
        deb: ast::DebNode<A>,
        definition_src: ast::Fun<A>,
    },
    DecreasingArgIndexTooBig {
        fun: ast::Fun<A>,
    },

    VconDefParamTypeFailsStrictPositivityCondition {
        def: ast::VconDef<A>,
        param_type_index: usize,
        normalized_param_type: NormalForm,
        path_from_param_type_to_problematic_deb: Vec<minimal_ast::NodeEdge>,
    },
    RecursiveIndParamAppearsInVconDefIndexArg {
        def: ast::VconDef<A>,
        index_arg_index: usize,
        normalized_index_arg: NormalForm,
        path_from_index_arg_to_problematic_deb: Vec<minimal_ast::NodeEdge>,
    },

    MatcheeTypeTypeIsErasableButReturnTypeTypeIsNotErasable {
        match_: ast::Match<A>,
        matchee_type_type: minimal_ast::UniverseNode,
        match_return_type_type: minimal_ast::UniverseNode,
    },
}

impl Debug for TypeError<UnitAuxData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}

impl Debug for TypeError<SpanAuxData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}

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
