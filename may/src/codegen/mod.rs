use zoc::syntax_tree::ast::prelude::minimal_ast as znode;

mod mnode {
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

use std::{collections::HashSet, rc::Rc};

mod cache_expr;

pub mod context;
pub use context::*;

mod convert_node_impls;

mod convert_param_defs_to_context_extension;
use convert_param_defs_to_context_extension::*;

mod cst_impls;

pub mod error;
pub use error::*;

#[cfg(test)]
mod tests;

/// If this function succeeds, it returns `Ok((converted_expr, sub_defs))` where:
///
/// - `converted_expr` is the Zo representation of `expr`
///
/// - `sub_defs` is a vector of the Zo representations of the
///   substitutable definitions (i.e., values defined by `fun`, `ind`, and `let`)
///   defined in the construction of `converted_expr`.
///
///   These definitions are in "top-to-bottom" order.
///
/// For example, if `expr` is `ind Foo case foo_x case foo_y return Set0 ind Bar case bar_x case bar_y return Set0 bar_x`,
/// then `converted_expr` is the Zo representation of `bar_x`,
/// and `sub_defs` is `vec![zo_foo, zo_bar]`
/// (where `zo_foo` and `zo_bar` are the Zo representations of `ind Foo ...` and `ind Bar ...` respectively).
pub fn may_to_zo(expr: &mnode::Expr) -> Result<(znode::Expr, Vec<znode::Expr>), SemanticError> {
    MayConverter::default().convert(expr, Context::empty())
}

#[derive(Debug, Default)]
struct MayConverter {
    znode_cache: NoHashHashMap<Digest, znode::Expr>,
    znode_vec_cache: NoHashHashMap<Digest, RcHashedVec<znode::Expr>>,
    str_val_cache: HashSet<Rc<StringValue>>,

    zo_typechecker: zoc::typecheck::TypeChecker,
}
