use zoc::syntax_tree::ast::prelude::minimal_ast::{self as znode, UnitAuxDataFamily};

mod jnode {
    pub use crate::{cst::*, token::*};
}

use zoc::syntax_tree::replace_debs::{DebUpshifter, ReplaceDebs};
use zoc::{
    hash::{Digest, GetDigest, NoHashHashMap},
    syntax_tree::ast::prelude::minimal_ast::{
        rc_hashed as bypass_cache_and_rc_hash, Deb, RcHashedVec, StringValue, Universe,
        UniverseLevel,
    },
};

type ZoError = zoc::typecheck::TypeError<UnitAuxDataFamily>;

use std::{collections::HashSet, rc::Rc};

mod cache_expr;

pub mod context;
pub use context::*;

mod conversion_impls;

// TODO: Support multifile compilation.
pub fn june_to_zo(expr: jnode::Expr) -> Result<znode::Expr, SemanticError> {
    JuneConverter::default().convert(&expr, Context::empty())
}

#[derive(Debug, Default)]
struct JuneConverter {
    znode_cache: NoHashHashMap<Digest, znode::Expr>,
    znode_vec_cache: NoHashHashMap<Digest, RcHashedVec<znode::Expr>>,
    str_val_cache: HashSet<Rc<StringValue>>,

    zo_typechecker: zoc::typecheck::TypeChecker,
}

#[derive(Clone, Debug)]
pub enum SemanticError {
    VarNotDefined(jnode::Ident),
    MultipleDecreasingParams(jnode::FunParamDef, jnode::FunParamDef),
    ConvertedExprHasZoErr(jnode::Expr, znode::Expr, ZoError),
}
