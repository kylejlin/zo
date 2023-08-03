use super::*;

#[derive(Clone, Debug)]
pub enum SemanticError {
    VarNotDefined(mnode::Ident),
    IllegalDashedParam(mnode::ParamDef),
}
