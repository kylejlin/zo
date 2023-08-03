use crate::cst as mnode;
use zoc::syntax_tree::ast as znode;

pub mod error;
pub use error::*;

pub fn generate_code(expr: mnode::Expr) -> Result<znode::Expr, SemanticError> {
    todo!()
}
