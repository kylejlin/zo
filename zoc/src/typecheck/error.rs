use super::*;

use crate::pretty_print::PrettyPrinted;

use std::fmt::{Debug, Result as FmtResult};

#[derive(Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: NumberLiteral,
        tcon_len: usize,
    },
    InvalidVconIndex(cst::Vcon),
    UnexpectedNonTypeExpression {
        expr: cst::Expr,
        type_: NormalForm,
    },
    UniverseInconsistencyInIndDef {
        index_or_param_type: cst::Expr,
        universe: Universe,
        ind: cst::Ind,
    },
    WrongNumberOfIndexArguments {
        def: cst::VconDef,
        expected: usize,
        actual: usize,
    },
    NonInductiveMatcheeType {
        expr: cst::Expr,
        type_: NormalForm,
    },
    WrongNumberOfMatchCases {
        match_: cst::Match,
        matchee_type_ind: Normalized<minimal_ast::Ind>,
    },
    WrongMatchReturnTypeArity {
        match_: cst::Match,
        matchee_type_args: Vec<NormalForm>,
    },
    WrongMatchCaseArity {
        actual_node: cst::NumberLiteral,
        expected: usize,
        match_: cst::Match,
        match_case_index: usize,
    },
    TypeMismatch {
        expr: cst::Expr,
        expected_type: NormalForm,
        actual_type: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: cst::App,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: cst::App,
        callee_type: Normalized<minimal_ast::For>,
        expected: usize,
        actual: usize,
    },
    FunHasZeroParams {
        fun: cst::Fun,
    },
    AppHasZeroArgs {
        app: cst::App,
    },
    ForHasZeroParams {
        for_: cst::For,
    },

    IllegalRecursiveCall {
        app: cst::App,
        callee_deb_definition_src: cst::Fun,
        required_decreasing_arg_index: usize,
        required_strict_superstruct: Deb,
    },
    RecursiveFunParamInNonCalleePosition {
        deb: cst::NumberLiteral,
        definition_src: cst::Fun,
    },
    DeclaredFunNonrecursiveButUsedRecursiveFunParam {
        deb: cst::NumberLiteral,
        definition_src: cst::Fun,
    },
    DecreasingArgIndexTooBig {
        fun: cst::Fun,
    },

    VconDefParamTypeFailsStrictPositivityCondition {
        def: cst::VconDef,
        param_type_index: usize,
        normalized_param_type: NormalForm,
        path_from_param_type_to_problematic_deb: Vec<minimal_ast::NodeEdge>,
    },
    RecursiveIndParamAppearsInVconDefIndexArg {
        def: cst::VconDef,
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
