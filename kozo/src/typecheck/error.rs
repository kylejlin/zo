use super::*;

#[derive(Debug, Clone)]
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
        level: UniverseLevel,
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
        matchee_type_ind: Normalized<ast::Ind>,
    },
    TypeMismatch {
        expr: cst::Expr,
        expected_type: NormalForm,
        actual_type: NormalForm,
        subbed_expected: NormalForm,
        subbed_actual: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: cst::App,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: cst::App,
        callee_type: Normalized<ast::For>,
        expected: usize,
        actual: usize,
    },
}
