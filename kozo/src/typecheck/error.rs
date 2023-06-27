use super::*;

#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: NumberLiteral,
        tcon_len: usize,
    },
    InvalidVconIndex(Vcon),
    UnexpectedNonTypeExpression {
        expr: Expr,
        type_: NormalForm,
    },
    UniverseInconsistencyInIndDef {
        index_or_param_type: Expr,
        level: UniverseLevel,
        ind: Ind,
    },
    WrongNumberOfIndexArguments {
        def: VconDef,
        expected: usize,
        actual: usize,
    },
    NonInductiveMatcheeType {
        expr: Expr,
        type_: NormalForm,
    },
    WrongNumberOfMatchCases {
        match_: Match,
        matchee_type_ind: Normalized<ast::Ind>,
    },
    TypeMismatch {
        expr: Expr,
        expected_type: NormalForm,
        actual_type: NormalForm,
        subbed_expected: NormalForm,
        subbed_actual: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: App,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: App,
        callee_type: Normalized<ast::For>,
        expected: usize,
        actual: usize,
    },
}
