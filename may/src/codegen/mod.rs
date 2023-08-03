use crate::cst as mnode;
use zoc::hash::{Digest, NoHashHashMap};
use zoc::syntax_tree::ast as znode;

pub mod error;
pub use error::*;

pub mod context;
pub use context::*;

pub fn may_to_zo(expr: &mnode::Expr) -> Result<znode::Expr, SemanticError> {
    MayConverter::default().convert(expr, Context::empty())
}

#[derive(Debug, Default)]
struct MayConverter {
    znode_cache: NoHashHashMap<Digest, znode::Expr>,
}

impl MayConverter {
    fn convert(&mut self, expr: &mnode::Expr, con: Context) -> Result<znode::Expr, SemanticError> {
        todo!()
    }
}
