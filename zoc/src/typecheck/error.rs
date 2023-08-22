use super::*;

use crate::pretty_print::PrettyPrinted;

use std::fmt::{Debug, Result as FmtResult};

#[derive(Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: NumberLiteral,
        tcon_len: usize,
    },
    InvalidVconIndex(ipist::Vcon),
    UnexpectedNonTypeExpression {
        expr: ipist::Expr,
        type_: NormalForm,
    },
    UniverseInconsistencyInIndDef {
        index_or_param_type: ipist::Expr,
        universe: Universe,
        ind: ipist::Ind,
    },
    WrongNumberOfIndexArguments {
        def: ipist::VconDef,
        expected: usize,
        actual: usize,
    },
    NonInductiveMatcheeType {
        expr: ipist::Expr,
        type_: NormalForm,
    },
    WrongNumberOfMatchCases {
        match_: ipist::Match,
        matchee_type_ind: Normalized<minimal_ast::Ind>,
    },
    WrongMatchReturnTypeArity {
        match_: ipist::Match,
        matchee_type_args: Vec<NormalForm>,
    },
    WrongMatchCaseArity {
        actual_node: ipist::NumberLiteral,
        expected: usize,
        match_: ipist::Match,
        match_case_index: usize,
    },
    TypeMismatch {
        expr: ipist::Expr,
        expected_type: NormalForm,
        actual_type: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: ipist::App,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: ipist::App,
        callee_type: Normalized<minimal_ast::For>,
        expected: usize,
        actual: usize,
    },
    FunHasZeroParams {
        fun: ipist::Fun,
    },
    AppHasZeroArgs {
        app: ipist::App,
    },
    ForHasZeroParams {
        for_: ipist::For,
    },

    IllegalRecursiveCall {
        app: ipist::App,
        callee_deb_definition_src: ipist::Fun,
        required_decreasing_arg_index: usize,
        required_strict_superstruct: Deb,
    },
    RecursiveFunParamInNonCalleePosition {
        deb: ipist::NumberLiteral,
        definition_src: ipist::Fun,
    },
    DeclaredFunNonrecursiveButUsedRecursiveFunParam {
        deb: ipist::NumberLiteral,
        definition_src: ipist::Fun,
    },
    DecreasingArgIndexTooBig {
        fun: ipist::Fun,
    },

    VconDefParamTypeFailsStrictPositivityCondition {
        def: ipist::VconDef,
        param_type_index: usize,
        normalized_param_type: NormalForm,
        path_from_param_type_to_problematic_deb: Vec<minimal_ast::NodeEdge>,
    },
    RecursiveIndParamAppearsInVconDefIndexArg {
        def: ipist::VconDef,
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
