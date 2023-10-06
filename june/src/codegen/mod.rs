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

/// If this function succeeds, it returns `Ok((converted_expr, toprightmost_defs))` where:
///
/// - `converted_leaf` is the Zo representation of `expr`
///
/// - `toprightmost_defs` is a vector of the Zo representations of the
///   toprightmost definitions in `expr`.
///
///   These definitions are in "top-to-bottom" order.
///
/// ## Example
///
/// Suppose we have the following code:
///
/// ```may
/// ind Bool
///     case false
///     case true
/// return Set0
///
/// // Overly complicated definition of `and` that uses DeMorgan's law.
/// fun and(x: Bool, y: Bool): Bool
///     fun not(a: Bool): Bool
///         match a
///         case false:
///             true
///         case true:
///             false
///         return1 Bool
///
///     fun or(a: Bool, b: Bool): Bool
///         match a
///         case false:
///             b
///         case true:
///             true
///         return1 Bool
///
///     // By DeMorgan's law:
///     not(or(not(x), not(y)))
///
/// and(true, true)
/// ```
///
/// In the above example, we can draw an AST like this
/// (details are omitted for brevity):
///
/// ```text
///                    [ind Bool]
///                       \
///                        \
///                       [fun and]
///                       /      \
///                      /        \
///                   [fun not]    \
///                   /  \          \
///                  /    \        {and(true, true)}
///          {match...}   [fun or]
///                       /      \
///                {match...}     {not(or(not(x), not(y)))}
/// ```
///
/// In the above AST, observe that:
///
/// Each definition
/// (i.e., `ind Bool`, `fun and`, `fun not`, and `fun or`) is a node.
/// The definition nodes are enclosed in square brackets ("[]").
///
/// Each definition's value (traditionally called the "right hand side")
/// is said definition's left child.
///
/// Each definition's dependency is said definition's right child.
///
/// Leaf values are enclosed in curly braces ("{}").
///
/// Observe that the toprightmost definitions are `ind Bool` and `fun and`,
/// since they are the nodes running down the right side of the AST
/// (we exclude the leaf `{and(true, true)}` because it's not a definition).
///
/// The definitions `fun not` and `fun or` are **not** toprightmost.
///
/// Therefore, if you call this function with the above example code,
/// `toprightmost_defs` will contain the Zo representations for `ind Bool` and `fun and`,
/// in that order.
pub fn may_to_zo(expr: &mnode::Expr) -> Result<(znode::Expr, Vec<znode::Expr>), SemanticError> {
    MayConverter::default().convert(expr, Context::empty(), &ContextToUnshiftedSubstitutableDefs)
}

trait ContextToOwned {
    type Out;
    fn convert_context_to_owned(&self, context: Context) -> Self::Out;
}

struct DropContext;
impl ContextToOwned for DropContext {
    type Out = ();

    fn convert_context_to_owned(&self, _: Context) -> Self::Out {}
}

struct ContextToUnshiftedSubstitutableDefs;
impl ContextToOwned for ContextToUnshiftedSubstitutableDefs {
    type Out = Vec<znode::Expr>;

    fn convert_context_to_owned(&self, context: Context) -> Self::Out {
        match context {
            Context::Base(entries) => {
                get_unshifted_substitutable_defs_from_entries(entries).collect()
            }

            Context::Snoc(rdc, rac) => {
                let mut rdc = self.convert_context_to_owned(*rdc);
                let rac = get_unshifted_substitutable_defs_from_entries(rac);
                rdc.extend(rac);
                rdc
            }
        }
    }
}

fn get_unshifted_substitutable_defs_from_entries<'a>(
    entries: &'a [UnshiftedEntry],
) -> impl 'a + Iterator<Item = znode::Expr> {
    entries.iter().filter_map(|entry| match entry.def_type {
        DefinitionType::Deb => None,
        DefinitionType::Substitutable => Some(entry.val.clone()),
    })
}

#[derive(Debug, Default)]
struct MayConverter {
    znode_cache: NoHashHashMap<Digest, znode::Expr>,
    znode_vec_cache: NoHashHashMap<Digest, RcHashedVec<znode::Expr>>,
    str_val_cache: HashSet<Rc<StringValue>>,

    zo_typechecker: zoc::typecheck::TypeChecker,
}
