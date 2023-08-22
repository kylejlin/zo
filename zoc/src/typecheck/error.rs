use super::*;

use crate::pretty_print::PrettyPrinted;

use std::fmt::{Debug, Result as FmtResult};

#[derive(Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: spanned_ast::DebNode,
        tcon_len: usize,
    },
    InvalidVconIndex(spanned_ast::Vcon),
    UnexpectedNonTypeExpression {
        expr: spanned_ast::Expr,
        type_: NormalForm,
    },
    UniverseInconsistencyInIndDef {
        index_or_param_type: spanned_ast::Expr,
        universe: Universe,
        ind: spanned_ast::Ind,
    },
    WrongNumberOfIndexArguments {
        def: spanned_ast::VconDef,
        expected: usize,
        actual: usize,
    },
    NonInductiveMatcheeType {
        expr: spanned_ast::Expr,
        type_: NormalForm,
    },
    WrongNumberOfMatchCases {
        match_: spanned_ast::Match,
        matchee_type_ind: Normalized<minimal_ast::Ind>,
    },
    WrongMatchReturnTypeArity {
        match_: spanned_ast::Match,
        matchee_type_args: Vec<NormalForm>,
    },
    WrongMatchCaseArity {
        stated_arity: usize,
        expected: usize,
        match_: spanned_ast::Match,
        match_case_index: usize,
    },
    TypeMismatch {
        expr: spanned_ast::Expr,
        expected_type: NormalForm,
        actual_type: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: spanned_ast::App,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: spanned_ast::App,
        callee_type: Normalized<minimal_ast::For>,
        expected: usize,
        actual: usize,
    },
    FunHasZeroParams {
        fun: spanned_ast::Fun,
    },
    AppHasZeroArgs {
        app: spanned_ast::App,
    },
    ForHasZeroParams {
        for_: spanned_ast::For,
    },

    IllegalRecursiveCall {
        app: spanned_ast::App,
        callee_deb_definition_src: spanned_ast::Fun,
        required_decreasing_arg_index: usize,
        required_strict_superstruct: Deb,
    },
    RecursiveFunParamInNonCalleePosition {
        deb: spanned_ast::DebNode,
        definition_src: spanned_ast::Fun,
    },
    DeclaredFunNonrecursiveButUsedRecursiveFunParam {
        deb: spanned_ast::DebNode,
        definition_src: spanned_ast::Fun,
    },
    DecreasingArgIndexTooBig {
        fun: spanned_ast::Fun,
    },

    VconDefParamTypeFailsStrictPositivityCondition {
        def: spanned_ast::VconDef,
        param_type_index: usize,
        normalized_param_type: NormalForm,
        path_from_param_type_to_problematic_deb: Vec<minimal_ast::NodeEdge>,
    },
    RecursiveIndParamAppearsInVconDefIndexArg {
        def: spanned_ast::VconDef,
        index_arg_index: usize,
        normalized_index_arg: NormalForm,
        path_from_index_arg_to_problematic_deb: Vec<minimal_ast::NodeEdge>,
    },
}

impl Debug for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
