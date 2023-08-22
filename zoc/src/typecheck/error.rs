use super::*;

use crate::pretty_print::PrettyPrinted;

use std::fmt::{Debug, Result as FmtResult};

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
}

impl Debug for TypeError<SpanAuxData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
