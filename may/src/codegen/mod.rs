use zoc::syntax_tree::ast as znode;

mod mnode {
    pub use crate::{cst::*, token::*};
}

use zoc::syntax_tree::replace_debs::{DebUpshifter, ReplaceDebs};
use zoc::{
    hash::{Digest, GetDigest, NoHashHashMap},
    syntax_tree::ast::{
        rc_hashed as bypass_cache_and_rc_hash, Deb, RcHashedVec, StringValue, UniverseLevel,
    },
};

use std::rc::Rc;

mod cache_expr;

pub mod context;
pub use context::*;

mod convert_node_impls;

mod convert_param_defs_to_context_extension;
use convert_param_defs_to_context_extension::*;

mod cst_impls;

pub mod error;
pub use error::*;

pub fn may_to_zo(expr: &mnode::Expr) -> Result<znode::Expr, SemanticError> {
    MayConverter::default().convert(expr, Context::empty())
}

#[derive(Debug, Default)]
struct MayConverter {
    znode_cache: NoHashHashMap<Digest, znode::Expr>,
    znode_vec_cache: NoHashHashMap<Digest, RcHashedVec<znode::Expr>>,

    zo_typechecker: zoc::typecheck::TypeChecker,
}
