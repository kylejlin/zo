use super::*;

#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidDeb {
        deb: RcSemHashed<DebNode>,
        tcon_len: usize,
    },
    InvalidVconIndex(RcSemHashed<Vcon>),
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
        match_: RcSemHashed<Match>,
        matchee_type_ind: Normalized<RcSemHashed<Ind>>,
    },
    TypeMismatch {
        expr: Expr,
        expected_type: NormalForm,
        actual_type: NormalForm,
        subbed_expected: NormalForm,
        subbed_actual: NormalForm,
    },
    CalleeTypeIsNotAForExpression {
        app: RcSemHashed<App>,
        callee_type: NormalForm,
    },
    WrongNumberOfAppArguments {
        app: RcSemHashed<App>,
        callee_type: Normalized<RcSemHashed<For>>,
        expected: usize,
        actual: usize,
    },
}
