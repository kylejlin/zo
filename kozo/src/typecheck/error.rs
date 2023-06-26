use super::*;

#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: RcHashed<DebNode>,
        tcon_len: usize,
    },
    InvalidVconIndex(RcHashed<Vcon>),
    UnexpectedNonTypeExpression {
        expr: Expr,
        type_: NormalForm,
    },
    UniverseInconsistencyInIndDef {
        expr: Expr,
        level: UniverseLevel,
        max_permitted_level: UniverseLevel,
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
        match_: RcHashed<Match>,
        matchee_type_ind: Normalized<RcHashed<Ind>>,
    },
    TypeMismatch {
        expr: Expr,
        expected_type: NormalForm,
        actual_type: NormalForm,
        subbed_expected: NormalForm,
        subbed_actual: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: RcHashed<App>,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: RcHashed<App>,
        callee_type: Normalized<RcHashed<For>>,
        expected: usize,
        actual: usize,
    },
}
