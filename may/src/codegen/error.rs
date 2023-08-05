use super::*;

#[derive(Clone, Debug)]
pub enum SemanticError {
    VarNotDefined(mnode::Ident),
    IllegalDashedParam(mnode::ParamDef),
    MultipleDashedParams(mnode::ParamDef, mnode::ParamDef),
    ReturnArityIsZero(mnode::ReturnArityLiteral),
    InvalidVconIndex(mnode::VconIndexLiteral),
}
